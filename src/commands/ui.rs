use super::css;
use crate::library::client::components::atoms;
use anyhow::Result;
use cliclack::{intro, outro, progress_bar, select, spinner, multi_progress};
use std::fs;
use std::io;
use std::path::Path;
use std::thread;

pub fn guided_ui() -> Result<()> {
    // todo:
    // split guide between CSS operations (modify/generate/install css theme tokens)
    // and module additions (CMS, CRM, Chat, Forum, etc)
    intro("Devano UI".to_string())?;
    let choices = [
        ("CSS", "CSS", "Modify/Generate/Install CSS Tokens"),
        ("Atoms", "Atoms", "Install all available atoms"),
        ("Modules", "Modules", "Install Modules"),
        ("Cancel", "Cancel", "Exit the UI command"),
    ];
    let choice = select("What would you like to do?")
        .items(&choices)
        .interact()?;
    match choice {
        "CSS" => {
            css_options()?;
        }
        "Atoms" => {
            install_atoms()?;
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
    let choice = select("What would you like to do?")
        .items(&choices)
        .interact()?;
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

pub fn install_atoms() -> Result<()> {
    intro("Installing all Atoms...".to_string())?;
    let all_atoms = atoms::registry::Atoms::all_variants();
    let multi = multi_progress("Cloning...");
    let pb1 = multi.add(progress_bar(all_atoms.len() as u64));
    pb1.start("JSX...");
    let client_thread = thread::spawn(move || -> Result<()> {
        for atom in all_atoms {
            atom.install()?;
            pb1.inc(1);
        }
        pb1.stop("Cloning Complete");
        pb1.clear();
        Ok(())
    });
    client_thread.join().unwrap()?;
    outro("Atoms complete!")?;
    Ok(())
}
