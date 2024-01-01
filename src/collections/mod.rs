mod collection_item;
mod imp;
use std::cell::RefCell;
use std::rc::Rc;

use glib::Object;
use gtk::{gio, SingleSelection};
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib::{self, clone, PropertyGet};

use crate::RUNTIME;
use crate::utils::collections::{get_all_collections, CollectionData};
use collection_item::CollectionItem;


glib::wrapper! {
    pub struct CollectionsWindow(ObjectSubclass<imp::CollectionsWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CollectionsWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup_collections(&self) {
        let collections = gio::ListStore::new::<CollectionItem>();
        self.imp()
            .collections
            .set(collections.clone())
            .expect("Could not set collections");

        let (sender, receiver) = async_channel::bounded(1);
    
        // Use Rc<RefCell<Vec<CollectionData>>> for shared ownership and interior mutability
        let collections_vec: Rc<RefCell<Vec<CollectionData>>> = Rc::new(RefCell::new(Vec::new()));
    
        // Clone Rc for the closure
        let collections_vec_clone = Rc::clone(&collections_vec);
    
        RUNTIME.spawn(async move {
            let collections = get_all_collections().await;
            sender.send(collections).await.expect("The channel needs to be open.");
        });
    
        glib::spawn_future_local(async move {
            while let Ok(result) = receiver.recv().await {
                match result {
                    Ok(data) => {
                        // Use the cloned Rc to access the shared data
                        collections_vec_clone.borrow_mut().extend(data);
                    }
                    Err(error) => {
                        println!("{:?}", error)
                    }
                }
            }
        });
    
        // Extract the Vec<CollectionData> from the Rc<RefCell<Vec<CollectionData>>>
        let extracted_collections = collections_vec.borrow();
        self.bind_collections_list(extracted_collections.to_vec())
    }

    fn collections(&self) -> gio::ListStore {
        self.imp()
        .collections
        .get()
        .expect("`collections` should be set in `setup_collections`.")
        .clone()
    }

    pub fn bind_collections_list(&self, collections_vec: Vec<CollectionData>) {
        // Convert `Vec<CollectionData>` to `Vec<CollectionItem>`
        let collections: Vec<CollectionItem> = collections_vec
        .into_iter()
        .map(|c| CollectionItem::new(
            &c.name, &c.id, 
        ))
        .collect();

        // Insert restored objects into model
        self.collections().extend_from_slice(&collections);

        let collections_store = gio::ListStore::new::<CollectionItem>();
        self.imp()
            .collections
            .set(collections_store.clone())
            .expect("Could not set collections");
        
    }
}
