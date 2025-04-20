use std::error::Error;

use rusqlite::Connection;
use slint::ComponentHandle;

use crate::{utils::crud::collections::get_all_collections, AppConfig, AppWindow};

pub fn check_startup_page(db: &Connection, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let collection_items = get_all_collections(db)?;
    let mut page: i32 = 1;

    if collection_items.len() > 0 {
        page = 2;
    }

    let config = app.global::<AppConfig>();
    config.set_page(page);

    Ok(())
}

pub fn process_page_change(app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    config.on_change_page(move |page| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        cfg.set_page(page);
    });

    Ok(())
}
