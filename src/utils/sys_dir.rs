use std::{error::Error, fs};

use crate::APP_ID;

pub fn get_db_path() -> Result<String, Box<dyn Error>> {
    // let mut path = glib::user_data_dir();
    // path.push(APP_ID);

    // // Create the directory if it doesn't exist
    // if !path.exists() {
    //     fs::create_dir_all(&path)?;
    // }

    // let file_path = path.join("querry.db");

    // // Convert PathBuf to String
    // let path_str = file_path
    //     .to_str()
    //     .ok_or("Invalid Unicode in path")?
    //     .to_string();

    // Ok(path_str)

    Ok("querry.db".to_string())
}

pub fn get_test_db_path() -> Result<String, Box<dyn Error>> {
    // let mut path = glib::user_data_dir();
    // path.push(APP_ID);

    // // Create the directory if it doesn't exist
    // if !path.exists() {
    //     fs::create_dir_all(&path)?;
    // }

    // let file_path = path.join("querry_test.db");

    // // Convert PathBuf to String
    // let path_str = file_path
    //     .to_str()
    //     .ok_or("Invalid Unicode in path")?
    //     .to_string();

    // Ok(path_str)

    Ok("querry_test.db".to_string())
}
