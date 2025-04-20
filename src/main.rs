// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use database::migrate_database;
use slint::{ComponentHandle, PlatformError};

mod database;
mod utils;

use lib::{
    callbacks::collections::{check_startup_page, process_page_change},
    database::get_database,
    AppConfig, AppWindow,
};

fn main() -> Result<(), PlatformError> {
    let db = get_database().expect("Could not connect to DB");
    migrate_database(&db).expect("Could not apply migrations");

    let app = AppWindow::new()?;

    check_startup_page(&db, &app).unwrap();
    process_page_change(&app).unwrap();

    app.run()
}
