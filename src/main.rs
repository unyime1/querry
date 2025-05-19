// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ComponentHandle, PhysicalSize, PlatformError};

use lib::{
    callbacks::{
        collections::{
            check_startup_page, load_collections, process_create_collection,
            process_get_collections, process_page_change, process_remove_collection,
            process_search_collections, process_update_collection,
        },
        images::process_get_images,
        requests::{
            process_create_requests, process_delete_request, process_get_requests,
            process_update_request,
        },
    },
    database::{get_database, migrate_database},
    AppWindow,
};

fn main() -> Result<(), PlatformError> {
    let db = get_database().expect("Could not connect to DB");

    migrate_database(db.clone()).expect("Could not apply migrations");

    let app = AppWindow::new()?;

    check_startup_page(db.clone(), &app).unwrap();
    load_collections(db.clone(), &app).unwrap();
    process_page_change(&app).unwrap();
    process_get_collections(db.clone(), &app).unwrap();
    process_create_collection(db.clone(), &app).unwrap();
    process_get_images(&app).unwrap();
    process_update_collection(db.clone(), &app).unwrap();
    process_remove_collection(db.clone(), &app).unwrap();
    process_search_collections(db.clone(), &app).unwrap();
    process_create_requests(db.clone(), &app).unwrap();
    process_get_requests(db.clone(), &app).unwrap();
    process_update_request(db.clone(), &app).unwrap();
    process_delete_request(db.clone(), &app).unwrap();

    let size: PhysicalSize = PhysicalSize::new(1920, 1080);
    app.set_window_height(size.height as f32);
    app.set_window_width(size.width as f32);

    app.run()?;
    Ok(())
}
