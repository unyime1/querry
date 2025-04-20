// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use slint::ComponentHandle;

mod database;
mod utils;

use lib::{AppConfig, AppWindow};

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    // Allow calling from the command line to
    // load a specified TODO file.
    let cfg = app.global::<AppConfig>();

    app.run()
}
