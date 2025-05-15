use std::process::Command;
use std::fs;
use std::path::Path;

pub fn install_devano_component() {
    
}

pub fn check_if_deps_installed(deps: &[&str]) -> std::io::Result<()> {
    if deps.is_empty() {
        return Ok(()); // nothing to check
    }

    let file_path = Path::new(".").join("src").join("devano");
    if !file_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {:?}", file_path),
        ));
    }

    let mut missing_deps: Vec<&str> = Vec::new();
    for entry in fs::read_dir(&file_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        for dep in deps {
            if file_name_str.contains(dep) {
                println!("✅ Found dependency in file: {}", file_name_str);
            } else {
                missing_deps.push(*dep);
            }
        }
    }

    if missing_deps.is_empty() {
        println!("✅ All dependencies are accounted for.");
        return Ok(()); // all dependencies are already present
    }

    install_devano_component(&missing_deps)?;

    Ok(())
}