use anyhow::{anyhow, bail, Result};

fn main() -> Result<()> {
    let config = dirs::config_dir().ok_or_else(|| anyhow!("No config dir"))?;

    let mut args = std::env::args().skip(1).take(2);

    let Some(template) = args.next() else {
        bail!("No template specified")
    };

    let Some(dest) = args.next() else {
        bail!("No destination specified")
    };

    let template_path = config.join("new").join(template);

    if !template_path.exists() {
        return Err(anyhow!("Template does not exist"));
    }
    if !template_path.is_dir() {
        return Err(anyhow!("Template is not a directory"));
    }

    let dest_dir = std::path::Path::new(&dest);

    if dest_dir.exists() {
        return Err(anyhow!("Destination already exists"));
    }

    walkdir::WalkDir::new(&template_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().strip_prefix(&template_path).unwrap().to_owned())
        .try_for_each(|file| {
            let template_file = template_path.join(&file);
            let dest_file = dest_dir.join(&file);
            if let Some(parent) = dest_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&template_file, &dest_file)?;
            Ok::<(), anyhow::Error>(())
        })?;

    Ok(())
}
