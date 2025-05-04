use std::{error::Error, rc::Rc};

use rusqlite::Connection;
use slint::{ComponentHandle, Model, VecModel};

use crate::{
    callbacks::images::load_image_item,
    utils::crud::collections::{create_collection, get_all_collections},
    AppConfig, AppWindow, CollectionItem,
};

/// Set page to view on app start.
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

/// Create load all collections on app start.
pub fn load_collections(db: &Connection, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let collection_items = get_all_collections(db)?;

    let mut collection_data: Vec<CollectionItem> = Vec::new();
    for collection_item in collection_items {
        let icon_item = match load_image_item(&collection_item.icon) {
            Ok(data) => data,
            Err(_) => {
                continue;
            }
        };
        collection_data.push(CollectionItem {
            id: collection_item.id.into(),
            name: collection_item.name.into(),
            icon: icon_item,
        });
    }
    let items_model = Rc::new(VecModel::from(collection_data));

    let config = app.global::<AppConfig>();
    config.set_collection_items(items_model.clone().into());

    Ok(())
}

/// Change page on ask
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

/// Get collections
pub fn process_get_collections(db: &Connection, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let collection_items = get_all_collections(db)?;

    let mut collection_data: Vec<CollectionItem> = Vec::new();

    for collection_item in collection_items {
        let icon_item = match load_image_item(&collection_item.icon) {
            Ok(data) => data,
            Err(_) => {
                continue;
            }
        };
        collection_data.push(CollectionItem {
            id: collection_item.id.into(),
            name: collection_item.name.into(),
            icon: icon_item,
        });
    }

    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    let items_model = Rc::new(VecModel::from(collection_data));
    config.on_get_collections(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        cfg.set_collection_items(items_model.clone().into());
    });

    Ok(())
}

/// Create a collection
pub fn process_create_collection(db: &Connection, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let new_collection = create_collection("New Collection".to_string(), &db)?;
    let icon_item = load_image_item(&new_collection.icon)?;
    let collection_item = CollectionItem {
        id: new_collection.id.into(),
        name: new_collection.name.into(),
        icon: icon_item,
    };

    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    config.on_create_collection(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let mut items: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();
        items.insert(0, collection_item.clone());
        println!("Collections count: {}", items.len());

        cfg.set_collection_items(Rc::new(VecModel::from(items)).into());
    });
    Ok(())
}
