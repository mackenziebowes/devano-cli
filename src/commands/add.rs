use crate::library::client::components::features::registry::Features;
use anyhow::Result;

pub fn add(args: &str) -> Result<()> {
    match args {
        "auth" => {
            Features::Auth.install()?;
            println!("Feature 'auth' installed successfully.");
        }
        _ => {
            println!("Unknown feature: {}", args);
        }
    }
    Ok(())
}
