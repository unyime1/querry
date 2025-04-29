use std::{error::Error, rc::Rc};

use rusqlite::Connection;
use slint::{ComponentHandle, Model, VecModel};

use crate::{
    utils::crud::collections::{create_collection, get_all_collections},
    AppConfig, AppWindow, CollectionItem,
};

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

pub fn process_get_collections(db: &Connection, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let collection_items = get_all_collections(db)?;

    let mut collection_data: Vec<CollectionItem> = Vec::new();

    for collection_item in collection_items {
        collection_data.push(CollectionItem {
            id: collection_item.id.into(),
            name: collection_item.name.into(),
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

pub fn process_create_collection(db: &Connection, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let new_collection = create_collection("New Collection".to_string(), &db)?;

    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    let collection_item = CollectionItem {
        id: new_collection.id.into(),
        name: new_collection.name.into(),
    };
    config.on_create_collection(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let mut items: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();
        items.insert(0, collection_item.clone());

        cfg.set_collection_items(Rc::new(VecModel::from(items)).into());
    });
    Ok(())
}
