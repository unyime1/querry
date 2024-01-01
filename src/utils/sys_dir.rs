use std::fs::{self, File};
use std::path::Path;

use crate::APP_ID;
use gtk::glib;

pub fn get_db_path() -> String {
    let mut path = glib::user_data_dir();
    path.push(APP_ID);

    // Create the directory if it doesn't exist
    if !path.exists() {
        fs::create_dir_all(&path).expect("Could not create directory.");
    }

    let file_path = path.join("querry.db");

    // Create the file only if it doesn't exist
    if !Path::new(&file_path).exists() {
        File::create(&file_path).expect("Could not create json file.");
    }

    file_path.to_str().unwrap().to_string()
}
