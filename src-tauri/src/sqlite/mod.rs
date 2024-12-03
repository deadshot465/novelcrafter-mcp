use crate::models::codex::{Codex, ParseError};
use crate::models::ClaudeDesktopConfig;
use std::borrow::Cow;
use std::fmt::Write;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

#[cfg(target_os = "macos")]
const CLAUDE_CONFIG_PATH: &str =
    "~/Library/Application Support/Claude/claude_desktop_config.json";

#[cfg(target_os = "windows")]
const CLAUDE_CONFIG_PATH: &str = "$APPDATA\\Claude\\claude_desktop_config.json";

pub fn create_tables(
    decompressed_path: &Path,
    db_path: &Path,
) -> anyhow::Result<sqlite::Connection> {
    let table_names = extract_table_names(decompressed_path)?;

    if db_path.exists() {
        std::fs::remove_file(db_path)?;
    }

    let mut query = "".to_string();
    for table_name in table_names.iter() {
        writeln!(
            &mut query,
            "CREATE TABLE {} (name TEXT, aliases TEXT, tags TEXT, content TEXT);",
            table_name
        )?;
    }

    let connection = sqlite::open(db_path)?;
    connection.execute(query)?;

    Ok(connection)
}

pub fn load_codices(
    decompressed_path: &Path,
    connection: &sqlite::Connection,
) -> anyhow::Result<()> {
    extract_table_names(decompressed_path)?;
    let entries = extract_entries(decompressed_path)?
        .into_iter()
        .map(parse_codex_file)
        .collect::<Result<Vec<_>, _>>()?;

    create_config_file()?;
    load_into_db(entries, connection)?;

    Ok(())
}

fn extract_table_names(decompressed_path: &Path) -> anyhow::Result<Vec<String>> {
    let table_names = std::fs::read_dir(decompressed_path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            if entry.path().is_dir() {
                Some(entry.file_name().to_str().unwrap_or_default().to_string())
            } else {
                None
            }
        })
        .map(|s| pluralizer::pluralize(&s.to_lowercase(), 1, false))
        .collect::<Vec<_>>();

    Ok(table_names)
}

fn extract_entries(decompressed_path: &Path) -> anyhow::Result<Vec<String>> {
    let read_dirs = std::fs::read_dir(decompressed_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path().read_dir())
        .collect::<Result<Vec<_>, _>>()
        .and_then(|dirs| {
            dirs.into_iter()
                .flat_map(|dir| {
                    dir.filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .map(|e| e.path().read_dir())
                })
                .collect::<Result<Vec<_>, _>>()
        });

    let entries = match read_dirs {
        Ok(dirs) => dirs
            .into_iter()
            .map(|dir| {
                dir.filter_map(|e| e.ok())
                    .find(|e| e.file_name().to_str().unwrap_or_default() == "entry.md")
            })
            .filter_map(|opt| {
                opt.map(|entry| std::fs::read_to_string(entry.path()).unwrap_or_default())
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Error when extracting entries: {}", e);
            vec![]
        }
    };

    Ok(entries)
}

fn parse_codex_file(codex: String) -> anyhow::Result<Codex> {
    let parts = codex.split("---").collect::<Vec<_>>();
    if parts.len() < 3 {
        return Err(anyhow::Error::new(ParseError::NoFrontmatter));
    }

    let yaml = YamlLoader::load_from_str(parts[1])
        .map_err(|e| anyhow::Error::new(ParseError::InvalidFrontmatter(e.to_string())))?;

    if yaml.is_empty() {
        return Err(anyhow::Error::new(ParseError::InvalidFrontmatter(
            "Frontmatter is empty".into(),
        )));
    }

    let front_matter = &yaml[0];
    let name = extract_string(front_matter, "name")?;
    let codex_type = extract_string(front_matter, "type")?;
    let aliases = extract_string_array(front_matter, "aliases");
    let tags = extract_string_array(front_matter, "tags");

    let content = parts[2..].join("---").trim().to_string();

    Ok(Codex {
        codex_type,
        name,
        aliases,
        tags,
        content,
    })
}

fn extract_string(yaml: &Yaml, field: &'static str) -> anyhow::Result<String> {
    yaml[field]
        .as_str()
        .map(String::from)
        .ok_or(anyhow::Error::new(ParseError::MissingRequiredField(field)))
}

fn extract_string_array(yaml: &Yaml, field: &'static str) -> Vec<String> {
    match &yaml[field] {
        Yaml::Array(arr) => arr
            .iter()
            .filter_map(|value| value.as_str())
            .map(String::from)
            .collect(),
        _ => Vec::new(),
    }
}

fn create_config_file() -> anyhow::Result<()> {
    let full_config_path = get_claude_mcp_config_path();
    let config_path = Path::new(full_config_path.as_ref());

    if !config_path.exists() {
        let new_config = ClaudeDesktopConfig::new();
        let serialized = serde_json::to_string_pretty(&new_config)?;
        std::fs::write(config_path, serialized)?
    }

    Ok(())
}

fn load_into_db(codices: Vec<Codex>, connection: &sqlite::Connection) -> anyhow::Result<()> {
    let mut query = "".to_string();

    for codex in codices.into_iter() {
        writeln!(
            &mut query,
            "INSERT INTO '{}' (name, aliases, tags, content) VALUES ('{}', '{}', '{}', '{}');",
            &codex.codex_type.replace('\'', "''"),
            &codex.name.replace('\'', "''"),
            &codex.aliases.join(", ").replace('\'', "''"),
            &codex.tags.join(", ").replace('\'', "''"),
            &codex.content.replace('\'', "''")
        )?;
    }

    connection.execute(&query)?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn get_claude_mcp_config_path() -> Cow<'static, str> {
    shellexpand::tilde(CLAUDE_CONFIG_PATH)
}

#[cfg(target_os = "windows")]
fn get_claude_mcp_config_path() -> Cow<'static, str> {
    shellexpand::env(CLAUDE_CONFIG_PATH).unwrap_or_default()
}