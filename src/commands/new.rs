use std::fs;
use std::path::{Path};
use crate::library::new::{client, server};
use std::process::{Command, Stdio};
use cliclack::{multi_progress, progress_bar, intro, input, outro};
use anyhow::Result;

pub fn create_devano_app() -> Result<()> {
    create_devano_app_internal()?;
    Ok(())
}

pub fn create_devano_app_internal() -> Result<()> {
    intro("New Devano Project")?;
    let project_name: String = input("What is this project called?").placeholder("devano").validate(|input: &String| {
        if input.is_empty() {
            Err("Please enter a project name.")
        } else {
            Ok(())
        }
    }).interact()?;
    let multi = multi_progress("Cloning Defaults...");
    let client_path = Path::new(&project_name).join("client");
    fs::create_dir_all(&client_path)?;
    let server_path = Path::new(&project_name).join("server");
    fs::create_dir_all(&server_path)?;
    let (client_files, client_commands) = client::make_files();
    let (server_files, server_commands) = server::make_files();
    let total_client_length = client_files.len() as u64 + client_commands.len() as u64;
    let total_server_length = server_files.len() as u64 + server_commands.len() as u64;
    let pb1 = multi.add(progress_bar(total_client_length));
    let pb2 = multi.add(progress_bar(total_server_length));
    pb1.start("Client Files...");
    pb2.start("Server Files...");
    for file in client_files {
        let mut file_path = client_path.clone();
        for folder in file.folder_tree {
            file_path = file_path.join(folder);
        }
        file_path = file_path.join(file.filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, file.contents)?;
        pb1.inc(1);
    }
    for command in client_commands {
        let _ = Command::new(command.command)
            .args(command.args)
            .current_dir(&client_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        pb1.inc(1);
    }
    for file in server_files {
        let mut file_path = server_path.clone();
        for folder in file.folder_tree {
            file_path = file_path.join(folder);
        }
        file_path = file_path.join(file.filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, file.contents)?;
        pb2.inc(1);
    }
    for command in server_commands {
        let _ = Command::new(command.command)
            .args(command.args)
            .current_dir(&server_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        pb2.inc(1);
    }
    pb1.stop("Client complete");
    pb2.stop("Server complete");
    outro("Install complete!")?;
    Ok(())
}

