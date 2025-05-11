use clap::Parser;           // Import the Parser trait for clap derive
mod cli;
mod util;
mod library;
mod commands;

fn main() {
    // Parse command-line arguments into our Cli struct
    let cli = cli::Cli::parse();

    // Handle subcommands
    match cli.command {
        cli::Commands::Ui => {
            if let Err(e) = commands::ui::guided_ui() {
                eprintln!("Error: failed to create app: {e}");
                std::process::exit(1);
            }
        }
        cli::Commands::New => {
            if let Err(e) = commands::new::create_devano_app() {
                eprintln!("Error: failed to create app: {e}");
                std::process::exit(1);
            }
        }
    }
}


    //    cli::Commands::DevAddComp { ref component } => {
    //         if component == "button" {
    //             // Attempt to create the component file
    //             if let Err(e) = commands::ui::create_button_component() {
    //                 eprintln!("Error: failed to add component: {e}");
    //                 std::process::exit(1);
    //             }
    //             println!("Created `components/devano/Button.tsx` successfully.");
    //         } else if component == "text-input" {
    //             // Attempt to create the component file
    //             if let Err(e) = commands::ui::create_text_input_component() {
    //                 eprintln!("Error: failed to add component: {e}");
    //                 std::process::exit(1);
    //             }
    //             println!("Created `components/devano/TextInput.tsx` successfully.");
    //         } 
    //         else {
    //             eprintln!("Error: unsupported component \"{}\"", component);
    //             std::process::exit(1);
    //         }
    //     }
    //     cli::Commands::DevAddCSS {ref part, ref theme} => {
    //         if part == "color" {
    //             if let Err(e) = commands::ui::add_color_css_vars(theme) {
    //                 eprintln!("Error: failed to add color: {e}");
    //                 std::process::exit(1);
    //             }
    //             println!("Imported color css {}.css successfully", theme)
    //         } else {
    //             eprintln!("Error: unsupported part \"{}\"", part);
    //             std::process::exit(1);
    //         }
    //     }
