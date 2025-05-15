use std::fs;
use std::path::Path;
use std::io::{self};
use anyhow::Result;
use super::{buttons::Buttons, inputs::Inputs, icons::Icons, utils::Utils, decorators::Decorators};
use crate::library::client::{writes, utils};

#[derive(Debug, Clone, Copy)]
pub enum Atoms {
    Buttons(Buttons),
    Icons(Icons),
    Utils(Utils),
    Inputs(Inputs),
    Decorators(Decorators),
}

impl Atoms {
    pub fn devano_dependencies(&self) -> &'static [Atoms] {
        match self {
            Atoms::Buttons(button) => match button {
                Buttons::Anchor => &[Atoms::Utils(Utils::Cn)],
                Buttons::AnchorButton => &[Atoms::Utils(Utils::Cn)],
                Buttons::Button => &[Atoms::Utils(Utils::Cn)],
                Buttons::ButtonAnchor => &[Atoms::Utils(Utils::Cn)],
                Buttons::IconButton => &[Atoms::Utils(Utils::Cn)],
            }
            Atoms::Icons(icon) => match icon {
                Icons::EyeClosed => &[Atoms::Utils(Utils::Cn)],
                Icons::EyeOpen => &[Atoms::Utils(Utils::Cn)],
            }
            Atoms::Inputs(input) => match input {
                Inputs::Text => &[Atoms::Utils(Utils::Cn)],
                Inputs::Password => &[Atoms::Utils(Utils::Cn), Atoms::Icons(Icons::EyeClosed), Atoms::Icons(Icons::EyeOpen), Atoms::Buttons(Buttons::IconButton)]
            }
            Atoms::Decorators(decorator) => match decorator {
                Decorators::Separators => &[Atoms::Utils(Utils::Cn)],
            }
            Atoms::Utils(util) => match util {
                Utils::Cn => &[],
            }
        }
    }

    pub fn get_ui(&self) -> &'static utils::utils::UiComponent {
        match self {
            Atoms::Buttons(button) => button.get_ui(),
            Atoms::Icons(icon) => icon.get_ui(),
            Atoms::Inputs(input) => input.get_ui(),
            Atoms::Utils(utils) => utils.get_ui(),
            Atoms::Decorators(decorators) => decorators.get_ui(),
        }
    }

    pub fn install(&self) -> Result<()> {
        let base_path = Path::new("client/src/devano");
        let ui_component = self.get_ui();
        let component_path = base_path.join(ui_component.folder_path); // Use folder_path
    
        // Recursively install dependencies
        for dependency in self.devano_dependencies() {
            dependency.install()?; // Recursively call `install` for each dependency
        }
    
        // Ensure the subfolder exists
        if !component_path.exists() {
            fs::create_dir_all(&component_path)?;
        }
    
        // Handle npm dependencies
        writes::npm::check_if_deps_installed(ui_component.npm_deps)?;
    
        let file_path = component_path.join(ui_component.filename); // Chain folder_path and filename
    
        if !file_path.exists() {
            // Write the file if it doesn't already exist
            fs::write(&file_path, ui_component.contents)?;
            println!("{} written to {:?}", ui_component.filename, file_path);
        } else {
            println!("{} already exists at {:?}", ui_component.filename, file_path);
        }
    
        Ok(())
    }
}