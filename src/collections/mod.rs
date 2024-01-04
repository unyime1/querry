mod collection_item;
mod imp;

use glib::Object;
use gtk::glib;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::BoxExt;
use gtk::{Box, Image, Label};

use crate::utils::collections::{prep_collections, CollectionData};
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
        let collections_vec =
            prep_collections().expect("An error occured while getting collections");
        self.bind_collections_list(collections_vec);
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
