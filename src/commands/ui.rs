use std::fs;
use std::io;
use std::path::Path;
use crate::library::client::components::atoms;
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
            outro("Not Yet Implemented - Exiting!")?;
            std::process::exit(0);
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
