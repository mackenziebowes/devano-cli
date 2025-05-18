use clap::Parser; // Import the Parser trait for clap derive
mod cli;
mod commands;
mod library;

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
        cli::Commands::Add(args) => {
            if let Err(e) = commands::add::add(&args.value) {
                println!("You Asked for: {}", args.value);
                eprintln!("Error: failed to process args: {e}");
                std::process::exit(1);
            }
        }
        cli::Commands::Feature => {
            if let Err(e) = commands::feature::guided_ui() {
                eprintln!("Error: failed to create app: {e}");
                std::process::exit(1);
            }
        }
    }
}
