use crate::library;
use anyhow::Result;

// The add function takes a slash separated list of arguments
// Different trees are accessed based on each position
// We can refactor this to use the proper `clap` system later 
// when all the warts are sorted.

pub fn add(args: &str) -> Result<()> {
    // split args by "/"
    let parts: Vec<&str> = args.split('/').collect();
    // match args[0]
    match parts.get(0) {
        Some(&"client") => {
            client(&parts[1..])?;
        }
        Some(&"server") => {
            // Handle tree2 logic
            println!("Calling server with args: {:?}", &parts[1..]);
        }
        Some(cmd) => {
            println!("Unknown cmd: {}", cmd);
        }
        None => {
            println!("No arguments provided.");
        }
    };
    Ok(())
}

pub fn client(args: &[&str]) -> Result<()> {
    match args.get(0) {
        Some(&"utils") => {
            println!("Calling client/utils with args: {:?}", &args[1..]);
        }
        Some(&"css") => {
            println!("Calling client/css with args: {:?}", &args[1..]);
        }
        Some(&"comp") => {
            println!("Calling client/comp with args: {:?}", &args[1..]);
        }
        Some(cmd) => {
            println!("Unknown cmd: {}", cmd)
        }
        None => {
            println!("No client arguments provided");
        }
    }
    Ok(())
}