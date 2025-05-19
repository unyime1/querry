use std::{error::Error, rc::Rc};

use rusqlite::Connection;
use slint::{ComponentHandle, Model, VecModel};

use crate::{
    utils::crud::requests::{
        create_request, delete_request, get_collection_requests, update_request_item, HTTPMethods,
        ProtocolTypes,
    },
    AppConfig, AppWindow, CollectionItem, RequestItem,
};

/// Get requests
pub fn process_get_requests(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    config.on_get_requests(move |collection_id| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let request_items = match get_collection_requests(db.clone(), &collection_id) {
            Ok(data) => data,
            Err(_) => [].to_vec(),
        };

        let request_data: Vec<RequestItem> = request_items
            .into_iter()
            .map(|item| RequestItem {
                id: item.id.into(),
                name: item.name.into(),
                url: item.url.unwrap_or("".to_string()).into(),
                protocol: item.protocol.into(),
                http_method: item.http_method.unwrap_or("get".to_string()).into(),
            })
            .collect();

        let items_model = Rc::new(VecModel::from(request_data));
        cfg.set_active_collection_requests(items_model.clone().into());
    });

    Ok(())
}

pub fn process_create_requests(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    config.on_create_request_item(move |collection_id, collection_index| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let request_item = match create_request(ProtocolTypes::Http, &collection_id, db.clone()) {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        let request_data: RequestItem = RequestItem {
            id: request_item.id.into(),
            name: request_item.name.into(),
            url: request_item.url.unwrap_or("".to_string()).into(),
            protocol: request_item.protocol.into(),
            http_method: request_item.http_method.unwrap_or("get".to_string()).into(),
        };

        let mut items: Vec<RequestItem> = cfg.get_active_collection_requests().iter().collect();
        items.insert(0, request_data);
        cfg.set_active_collection_requests(Rc::new(VecModel::from(items)).into());

        // Get collection and increase request count.
        let mut items: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();

        if let Some(item_ref) = items.get_mut(collection_index as usize) {
            item_ref.request_count += 1;
        }
        cfg.set_collection_items(Rc::new(VecModel::from(items)).into());
    });

    Ok(())
}

pub fn process_update_request(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    config.on_update_request_item(move |request_id, name, protocol, http_method, url, index| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        let request_item = match update_request_item(
            &request_id,
            Some(&name),
            ProtocolTypes::from_string(&protocol),
            HTTPMethods::from_string(&http_method),
            Some(&url),
            db.clone(),
        ) {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        let request_data: RequestItem = RequestItem {
            id: request_item.id.into(),
            name: request_item.name.into(),
            url: request_item.url.unwrap_or("".to_string()).into(),
            protocol: request_item.protocol.into(),
            http_method: request_item.http_method.unwrap_or("get".to_string()).into(),
        };

        let mut items: Vec<RequestItem> = cfg.get_active_collection_requests().iter().collect();

        if let Some(item_ref) = items.get_mut(index as usize) {
            *item_ref = request_data;
        }
        cfg.set_active_collection_requests(Rc::new(VecModel::from(items)).into());
    });

    Ok(())
}

pub fn process_delete_request(db: Rc<Connection>, app: &AppWindow) -> Result<(), Box<dyn Error>> {
    let config = app.global::<AppConfig>();
    let weak_app = app.as_weak();

    config.on_remove_request_item(move |request_id, request_index, collection_index| {
        let app = weak_app.upgrade().unwrap();
        let cfg = app.global::<AppConfig>();

        match delete_request(&request_id, db.clone()) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error deleting request  - {}", error);
                return;
            }
        };

        let mut items: Vec<RequestItem> = cfg.get_active_collection_requests().iter().collect();

        if items.get_mut(request_index as usize).is_some() {
            items.remove(request_index as usize);
        }
        cfg.set_active_collection_requests(Rc::new(VecModel::from(items)).into());

        // Get collection and increase request count.
        let mut collections: Vec<CollectionItem> = cfg.get_collection_items().iter().collect();

        if let Some(item_ref) = collections.get_mut(collection_index as usize) {
            item_ref.request_count -= 1;
        }
        cfg.set_collection_items(Rc::new(VecModel::from(collections)).into());
    });

    Ok(())
}
