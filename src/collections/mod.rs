mod collection_item;
mod imp;

use std::cell::RefCell;
use std::rc::Rc;

use glib::Object;
use gtk::glib;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::BoxExt;
use gtk::{Box, Image, Label};

use crate::utils::collections::{get_all_collections, CollectionData};
use crate::RUNTIME;
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

        // Use Rc<RefCell<Vec<CollectionData>>> for shared ownership and interior mutability
        let collections_vec: Rc<RefCell<Vec<CollectionData>>> = Rc::new(RefCell::new(Vec::new()));

        // Clone Rc for the closure
        let collections_vec_clone = Rc::clone(&collections_vec);

        RUNTIME.spawn(async move {
            let collections = get_all_collections().await;
            sender
                .send(collections)
                .await
                .expect("The channel needs to be open.");
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

    pub fn bind_collections_list(&self, mut collections_vec: Vec<CollectionData>) {
        collections_vec.push(CollectionData {
            id: "hi".to_string(),
            name: "Test".to_string(),
        });

        // Convert `Vec<CollectionData>` to `Vec<CollectionItem>`
        let collections: Vec<CollectionItem> = collections_vec
            .into_iter()
            .map(|c| CollectionItem::new(&c.name, &c.id))
            .collect();

        let collection_list = self.imp().collections_list.clone();
        for collection_item in collections {
            let box_widget = Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .hexpand(true)
                .visible(true)
                .build();

            let image_widget = Image::builder()
                .icon_name("folder-drag-accept-symbolic")
                .build();

            box_widget.append(&image_widget);

            let label_widget = Label::builder().label(collection_item.name()).build();
            box_widget.append(&label_widget);

            collection_list.append(&box_widget);
        }
    }
}
