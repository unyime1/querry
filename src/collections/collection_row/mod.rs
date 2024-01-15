mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;

use super::collection_item::CollectionItem;
use crate::utils::collections::delete_collection;

glib::wrapper! {
    pub struct CollectionRow(ObjectSubclass<imp::CollectionRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for CollectionRow {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn delete_collection(&self) {
        let collection_label = self.imp().collection_label.clone();
        let collection_id = self.imp().collection_id.clone();
        println!("Deleted {}", collection_label.label());
    }

    pub fn bind(&self, collection_item: &CollectionItem) {
        // Get state
        let collection_icon = self.imp().collection_icon.get();
        let collection_label = self.imp().collection_label.get();
     
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `collection_item.name` to `collection_row.collection_label.label`
        let collection_label_binding = collection_item
            .bind_property("name", &collection_label, "label")
            .bidirectional()
            .sync_create()
            .build();
        // Save binding
        bindings.push(collection_label_binding);

        // Bind `collection_item.icon` to `collection_row.collection_icon.icon_name`
        let collection_icon_binding = collection_item
            .bind_property("icon", &collection_icon, "icon-name")
            .sync_create()
            .build();
        // Save binding
        bindings.push(collection_icon_binding);

    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
