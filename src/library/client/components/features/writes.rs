use crate::library::client::utils::utils::{ApiComponent, write_file};
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::Path;

fn api_path() -> &'static Path {
    Path::new("client/src/devano/api")
}

pub fn update_api_index(module_name: &str, module_path: &str) -> Result<()> {
    let index_path = api_path().join("index.ts");
    let mut content = if index_path.exists() {
        fs::read_to_string(&index_path)?
    } else {
        String::new()
    };

    // Check if the module is already imported
    let import_statement = format!("import {{ {} }} from \"./{}\";", module_name, module_path);
    if content.contains(&import_statement) {
        return Ok(());
    }

    // Add the import statement at the top
    let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    lines.insert(0, import_statement);
    // Find the `api` object and insert the module
    if let Some(api_index) = lines
        .iter()
        .position(|line| line.trim() == "export const api = {")
    {
        lines.insert(api_index + 1, format!("\t{},", module_name));
    } else {
        return Err(anyhow::anyhow!("Could not find `api` object in index.ts"));
    }

    // Write the updated content back to the file
    let updated_content = lines.join("\n");
    let mut file = fs::File::create(&index_path)?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

/// Installs an API module to "client/src/devano/api"
pub fn add_api(api: &ApiComponent) -> Result<()> {
    let module_path = api_path().join(api.folder_path);
    let file_path = module_path.join(api.filename);
    write_file(&file_path, api.contents)?;
    update_api_index(api.module_name, api.folder_path)?;
    Ok(())
}
