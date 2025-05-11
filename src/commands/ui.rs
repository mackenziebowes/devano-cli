use std::fs;
use std::io;
use std::path::Path;
use crate::library::components::ui;
use crate::library;
use cliclack::{select, intro, spinner, outro};
use anyhow::Result;
use super::css;

pub fn guided_ui() -> Result<()> {
    // todo:
    // split guide between CSS operations (modify/generate/install css theme tokens)
    // and module additions (CMS, CRM, Chat, Forum, etc)
    intro("Devano UI".to_string())?;
    let choices = [
        ("CSS", "CSS", "Modify/Generate/Install CSS Tokens"),
        ("Modules", "Modules", "Install Modules"),
        ("Cancel", "Cancel", "Exit the UI command"),
    ];
    let choice = select("What would you like to do?").items(&choices).interact()?;
    match choice {
        "CSS" => {
            css_options()?;
        }
        "Modules" => {
            list_components()?;
        }
        "Cancel" => {
            outro("Exiting Devano UI")?;
            std::process::exit(0);
        }
        _ => unreachable!(),
    };
    Ok(())
}

pub fn css_options() -> Result<()> {
    intro("Devano CSS".to_string())?;
    let choices = [
        ("Color", "Color", "Modify/Generate/Install color tokens"),
        ("Go Back", "Go Back", "Return to the top menu"),
    ];
    let choice = select("What would you like to do?").items(&choices).interact()?;
    match choice {
        "Color" => {
            css::add_guided_palette()?;
        }
        "Go Back" => {
            guided_ui()?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub fn create_component(component_name: &str) -> Result<()> {
    intro(format!("Installing Devano Component {}", component_name))?;
    let spinner = spinner();
    spinner.start("Installing");
    let registry = ui::registry::make_component_registry();
    // Lookup the component definition
    let component = registry.get(component_name).ok_or_else(|| {
        spinner.error("Unkown Component");
        let _ = outro("Installation Failed");
        io::Error::new(io::ErrorKind::NotFound, format!("Component '{}' not found in registry", component_name))
    })?;
    // Ensure the target directory exists
    let dir_path = Path::new("src").join("components").join("devano");
    fs::create_dir_all(&dir_path)?;
    // Determine the file name and content based on the component name
    let file_name = format!("{}", component.filename);
    let file_path = dir_path.join(&file_name);

    // Write the component file
    fs::write(&file_path, component.contents)?;
    spinner.stop(format!("./src/components/devano/{} created!", component.filename));
    // do the deps
    spinner.start("Installing NPM Packages");
    library::utils::npm::check_if_deps_installed(component.required_npm)?;
    spinner.stop("Install complete!");
    Ok(())
}

pub fn list_components() -> Result<()> {
    let registry = ui::registry::make_component_registry();
    let mut items: Vec<(String, String, String)> = Vec::new(); // "value" | "label" (visible in list) | "hint" (displays on hover)
    for (_key, component) in &registry {
        items.push((format!("{}", component.name), format!("{}", component.name), format!("{}", component.description)))
    }
    let selected = select("Select an item")
        .items(&items)
        .interact()?;
    create_component(&selected)?;
    Ok(())
}
