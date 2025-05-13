use std::{error::Error, rc::Rc};

use rusqlite::Connection;
use slint::{ComponentHandle, Model, VecModel};

use crate::{
    callbacks::images::load_image_item,
    utils::crud::collections::{
        create_collection, delete_collection, get_all_collections, update_collection_item,
    },
    AppConfig, AppWindow, CollectionItem,
};

/// Set page to view on app start.
pub fn check_startup_page(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
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
pub fn load_collections(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
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
            icon_name: collection_item.icon.into(),
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
pub fn process_get_collections(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
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
            icon_name: collection_item.icon.into(),
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
pub fn process_create_collection(
    db: Rc<Connection>,
    app: &AppWindow,
) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    let db_for_closure = db.clone();
    config.on_create_collection(move || {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let new_collection =
            match create_collection("New Collection".to_string(), db_for_closure.clone()) {
                Ok(data) => data,
                Err(error) => {
                    eprintln!("Error Creating collection  - {}", error);
                    return;
                }
            };
        let icon_item = match load_image_item(&new_collection.icon) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Error loading image  - {}", error);
                return;
            }
        };
        let collection_item = CollectionItem {
            id: new_collection.id.into(),
            name: new_collection.name.into(),
            icon: icon_item,
            icon_name: new_collection.icon.into(),
        };

        let mut items: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();
        items.insert(0, collection_item.clone());
        cfg.set_collection_items(Rc::new(VecModel::from(items)).into());
    });
    Ok(())
}

/// Update a collection
pub fn process_update_collection(
    db: Rc<Connection>,
    app: &AppWindow,
) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    let db_for_closure = db.clone();
    config.on_update_collection(move |id, name, icon, index| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let new_collection = match update_collection_item(&id, &name, &icon, db_for_closure.clone())
        {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Error updating collection  - {}", error);
                return;
            }
        };
        let icon_item = match load_image_item(&new_collection.icon) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Error loading image  - {}", error);
                return;
            }
        };
        let collection_item = CollectionItem {
            id: new_collection.id.into(),
            name: new_collection.name.into(),
            icon: icon_item,
            icon_name: new_collection.icon.into(),
        };

        let mut items: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();
        if let Some(item_ref) = items.get_mut(index as usize) {
            *item_ref = collection_item;
        }
        cfg.set_collection_items(Rc::new(VecModel::from(items)).into());
    });
    Ok(())
}

/// Update a collection
pub fn process_remove_collection(
    db: Rc<Connection>,
    app: &AppWindow,
) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    let db_for_closure = db.clone();
    config.on_remove_collection(move |id, index| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        match delete_collection(&id, db_for_closure.clone()) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error deleting collection  - {}", error);
                return;
            }
        };
        let mut items: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();
        if let Some(_) = items.get_mut(index as usize) {
            items.remove(index as usize);
        }
        cfg.set_collection_items(Rc::new(VecModel::from(items)).into());
    });
    Ok(())
}
