use std::io::Cursor;
use std::path::Path;

pub fn decompress(zip_file: &[u8]) -> anyhow::Result<String> {
    let cursor = Cursor::new(zip_file);
    let mut archive = zip::ZipArchive::new(cursor)?;
    let temp_path = format!(
        "{}/{}",
        std::env::temp_dir().to_str().unwrap_or_default(),
        "novelcrafter"
    );
    println!("Temp Path: {}", &temp_path);

    {
        let path = Path::new(&temp_path);
        if path.exists() {
            std::fs::remove_dir_all(path)?;
        }
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = match file.enclosed_name() {
            None => continue,
            Some(path) => path,
        };

        let actual_path = format!("{}/{}", &temp_path, out_path.to_str().unwrap_or_default());

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, &actual_path);
            std::fs::create_dir_all(&actual_path)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                &actual_path,
                file.size()
            );

            if let Some(p) = out_path.parent() {
                let full_path = format!("{}/{}", &temp_path, p.to_str().unwrap_or_default());
                let parent_path = Path::new(&full_path);
                if !parent_path.exists() {
                    std::fs::create_dir_all(parent_path)?;
                }
            }

            let mut outfile = std::fs::File::create(&actual_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(temp_path)
}
