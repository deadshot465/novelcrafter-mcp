pub mod codex;

use serde::{Deserialize, Serialize};

pub const MCP_DB_DEFAULT_PATH: &str = "~/novelcrafter.db";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sqlite {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpConfig {
    pub sqlite: Sqlite,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaudeDesktopConfig {
    pub mcp_servers: McpConfig,
}

impl ClaudeDesktopConfig {
    pub fn new() -> Self {
        let db_path = shellexpand::tilde(MCP_DB_DEFAULT_PATH);

        ClaudeDesktopConfig {
            mcp_servers: McpConfig {
                sqlite: Sqlite {
                    command: "uvx".to_string(),
                    args: vec![
                        "mcp-server-sqlite".into(),
                        "--db-path".into(),
                        db_path.to_string(),
                    ],
                },
            },
        }
    }
}
