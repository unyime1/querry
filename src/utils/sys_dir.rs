use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};

use directories::ProjectDirs;

/// Get the database path based on whether it's a test or not.
pub fn get_db_path(is_test: Option<bool>) -> Result<String, Box<dyn Error>> {
    let file_path: PathBuf;

    let is_test = is_test.unwrap_or(false);
    if is_test {
        // If it's a test, use a temporary directory
        file_path = std::env::temp_dir().join("querry_test.db");
    } else {
        // For non-test cases, use the user data directory
        let project_dirs = ProjectDirs::from("org", "etim", "querry")
            .ok_or("Unable to get project directories")?;
        file_path = project_dirs.data_dir().join("querry.db");
    }

    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create the file if it doesn't exist
    if !file_path.exists() {
        File::create(&file_path)?; // This creates an empty file
    }

    let path_str = file_path
        .to_str()
        .ok_or("Invalid Unicode in path")?
        .to_string();
    Ok(path_str)
}
