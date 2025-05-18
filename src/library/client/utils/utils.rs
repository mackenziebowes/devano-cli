// Structs

/// Represents a UI component with metadata and content details.
///
/// # Fields
/// - `name`: The name of the UI component.
/// - `description`: A brief description of the UI component.
/// - `long_description`: A more detailed description of the UI component. This field may not always be used.
/// - `filename`: The name of the file associated with the UI component.
/// - `contents`: The contents of the file associated with the UI component.
/// - `folder_path`: The folder path where the UI component resides.
/// - `npm_deps`: A list of NPM dependencies required by the UI component.
pub struct UiComponent {
    pub name: &'static str,
    pub description: &'static str,
    #[allow(dead_code)]
    pub long_description: &'static str,
    pub filename: &'static str,
    pub contents: &'static str,
    pub folder_path: &'static str,
    pub npm_deps: &'static [&'static str],
}

/// Represents a component of an API, containing metadata and content information.
///
/// # Fields
/// - `module_name`: The name of the module associated with this API component.
/// - `description`: A brief description of the API component.
/// - `long_description`: A more detailed description of the API component. This field may not always be used.
/// - `filename`: The name of the file associated with this API component.
/// - `contents`: The contents of the file as a static string.
/// - `folder_path`: The path to the folder containing the file associated with this API component.
pub struct ApiComponent {
    pub module_name: &'static str,
    pub description: &'static str,
    #[allow(dead_code)]
    pub long_description: &'static str,
    // at this stage, always say "index.ts" for... reasons
    pub filename: &'static str,
    // the payload written to your project
    // at this stage, keep all related feature logic to a single file
    pub contents: &'static str,
    // declare the path from "<project>/client/src/devano/api" - eg, "auth" is fine
    // passing "auth" will create "<project>/client/src/devano/api/auth"
    // and in "<project>/client/src/devano/api/index.ts" it will import from "./auth"
    pub folder_path: &'static str,
}

// Functions

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Writes the specified contents to a file at the given file path.
///
/// If the parent directory of the file path does not exist, it will be created.
/// If the file already exists, the function will not overwrite it and will log a message indicating that the file already exists.
///
/// # Arguments
///
/// * `file_path` - A reference to a `Path` representing the file path where the contents should be written.
/// * `contents` - A string slice containing the contents to be written to the file.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the operation is successful, or an error wrapped in `anyhow::Result` if an error occurs.
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let file_path = Path::new("example.txt");
///     let contents = "Hello, world!";
///     write_file(file_path, contents)?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The parent directory cannot be created.
/// - The file cannot be written due to insufficient permissions or other I/O errors.
///
/// # Notes
///
/// This function uses `fs::create_dir_all` to create parent directories if they do not exist,
/// and `fs::write` to write the file contents.
pub fn write_file(file_path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if !file_path.exists() {
        fs::write(file_path, contents)?;
    }
    Ok(())
}
