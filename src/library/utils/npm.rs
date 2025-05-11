use std::process::Command;
use std::fs;
use std::path::Path;

pub fn install_npm_deps(deps: &[&str]) -> std::io::Result<()> {
    if deps.is_empty() {
        return Ok(()); // nothing to do
    }

    let status = Command::new("pnpm")
        .args(std::iter::once("add").chain(deps.iter().copied()))
        .status()?; // executes the command

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to install dependencies: {:?}", deps),
        ));
    }

    println!("✅ Installed: {}", deps.join(", "));
    Ok(())
}

pub fn check_if_deps_installed(deps: &[&str]) -> std::io::Result<()> {
    if deps.is_empty() {
        return Ok(()); // nothing to check
    }

    let file_path = Path::new(".").join("package.json");
    let contents = fs::read_to_string(&file_path)?;
    let mut missing_deps: Vec<&str> = Vec::new();

    for dep in deps {
        if !contents.contains(dep) {
            missing_deps.push(*dep);
        }
    }

    if missing_deps.is_empty() {
        println!("✅ Already Installed: {}", deps.join(", "));
        return Ok(()); // all dependencies are already installed
    }

    install_npm_deps(&missing_deps)?;

    Ok(())
}