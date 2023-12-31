mod collection_item;
mod imp;

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
        let (sender, receiver) = async_channel::bounded(1);
        RUNTIME.spawn(clone!(@strong sender => async move {
            let collections = get_all_collections().await;
            sender.send(collections).await.expect("The channel needs to be open.");
        }));

        glib::spawn_future_local(async move {
            while let Ok(result) = receiver.recv().await {
                match result {
                    Ok(data) => {
                        self.bind_collections_list(&data);
                    }
                    Err(error) => {
                        println!("{:?}", error)
                    }
                }
            }
        });
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
