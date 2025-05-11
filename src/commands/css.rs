use std::io;
use crate::library;
use crate::library::colors::named_palettes::NamedPalette;
use crate::library::colors::writes::{write_devano_palette_css, write_devano_palette_rust};
use cliclack::{select, intro, input, outro};
use strum::IntoEnumIterator;
use anyhow::Result;

pub fn add_guided_palette() -> Result<()> {
    intro("Devano Palette Tools".to_string())?;

    let choices = [
        ("Make a new palette", "Make a new palette", "Open the guided palette maker"), ("Add a named palette", "Add a named palette", "Open the guided palette selector"), 
        ("Cancel", "Cancel", "Exit the program")
        ];
    let choice = select("What would you like to do?").items(&choices).interact()?;

    match choice {
        "Make a new palette" => {
            split_complexity()?;
        },
        "Add a named palette" => {
            add_named_palette()?;
        },
        "Cancel" => {
            outro("Operation canceled.")?;
        },
        _ => unreachable!(),
    };
    Ok(())
}


pub fn add_named_palette() -> Result<()> {
    // Step 1: List available named palettes
    let palette_names: Vec<(&str, &str, &str)> = NamedPalette::iter()
        .map(|palette| {
            let name = palette.as_str();
            (name, name, palette.description())
        })
        .collect();

    // Step 2: Prompt the user to select a palette
    let selected_palette_name = select("Select a named palette:")
        .items(&palette_names)
        .interact()?;

    // Step 3: Convert the selected name to a NamedPalette variant
    if let Some(named_palette) = NamedPalette::from_str(&selected_palette_name) {
        // Step 4: Get the corresponding DevanoPalette
        let palette = named_palette.get_palette();

        // Step 5: Ask the user how they want to export the palette
        let export_options = get_export_options();
        let export_choice = select("How would you like to export the palette?")
            .items(&export_options)
            .interact()?;

        match export_choice {
            "Export to Rust" => {
                write_devano_palette_rust(&palette)?;
            }
            "Export to CSS" => {
                write_devano_palette_css(&palette)?;
            }
            "Cancel" => {
            }
            _ => unreachable!(),
        };
    } else {
        eprintln!("Invalid palette selection.");
    };
    Ok(())
}


pub fn split_complexity() -> Result<()> {
    let complexity_levels = [
        ("Simple", "Simple", "Make a full 32-token palette from just one color"), 
        ("Standard", "Standard", "Not yet implemented - will allow making a 32-token palette from three colors"), 
        ("Sophisticated", "Sophisticated", "Not yet implemented - hand type all 32 color tokens"), ("Go Back", "Go Back", "Go up one level")];
    let complexity = select("Choose the complexity level:").items(&complexity_levels).interact()?;

    match complexity {
        "Simple" => {
            split_simple_export()?;
        },
        "Standard" => {
            outro("Standard complexity is not yet implemented.")?;
        },
        "Sophisticated" => {
            outro("Sophisticated complexity is not yet implemented.")?;
        },
        "Go Back" => {
            add_guided_palette()?;
        },
        _ => unreachable!(),
    };
    Ok(())
}

#[derive(Debug)]
pub enum Destination {
    Rust,
    Css,
}

fn get_export_options() -> [(&'static str, &'static str, &'static str); 3] {
    [
        ("Export to Rust", "Export to Rust", "Exports the 32-token theme to ./src/codegen/palettes/newest.rs"),
        ("Export to CSS", "Export to CSS", "Exports the 32-token theme to ./src/css/devano/devano.css"),
        ("Go Back", "Go Back", "Go up one level"),
    ]
}

pub fn split_simple_export() -> Result<()> {
    let export_options = get_export_options();
    let export_choice = select("How would you like to export the palette?").items(&export_options).interact()?;
    match export_choice {
        "Export to Rust" => { 
            add_simple_palette(Destination::Rust)?;
        }
        "Export to CSS" => {
            add_simple_palette(Destination::Css)?;
        }
        "Go Back" => {
            split_complexity()?;
        },
        _ => unreachable!(),
    };

    Ok(())
}

pub fn get_hex_code() -> Result<String> {
    let hex_regex = regex::Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$")
    .expect("Failed to compile regex");
    let code = input("Give us a hex-code:")
        .placeholder("#FDC0C0")
        .validate(move |input: &String| {
            if input.is_empty() {
                Err("Please enter a hex code.")
            } else if !hex_regex.is_match(input) {
                Err("Please enter a valid hex-code (e.g., #RRGGBB or #RGB).")
            } else {
                Ok(())
            }
        })
        .interact()?;
    Ok(code)
}

pub fn add_simple_palette(dest: Destination) -> Result<()> {
    let color = get_hex_code()?;
    let palette = library::colors::transforms::make_simple_devano_palette(&color)
        .map_err(|e| {
            eprintln!("Failed to create palette: {}", e);
            io::Error::new(io::ErrorKind::Other, "Failed to create palette")
        })?;
    match dest {
        Destination::Rust => {
            library::colors::writes::write_devano_palette_rust(&palette).map_err(|e| {
                eprintln!("Failed to write palette: {}", e);
                io::Error::new(io::ErrorKind::Other, "Failed to write palette")
            })?;
        }
        Destination::Css => {
            library::colors::writes::write_devano_palette_css(&palette).map_err(|e| {
                eprintln!("Failed to write palette: {}", e);
                io::Error::new(io::ErrorKind::Other, "Failed to write palette")
            })?;
        },
    };
    Ok(())
}