mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;

use super::request_item::RequestItem;

glib::wrapper! {
    pub struct RequestRow(ObjectSubclass<imp::RequestRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for RequestRow {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, request_item: &RequestItem) {
        // Get state
        let name = self.imp().name.get();
        let request_logo = self.imp().request_icon.get();

        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind name.
        let name_binding = request_item
            .bind_property("name", &name, "label")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(name_binding);

        // Bind icon
        let icon_binding = request_item
            .bind_property("icon", &request_logo, "file")
            .sync_create()
            .build();
        bindings.push(icon_binding);
    }

    pub fn set_request_id(&self, new_id: String) {
        *self.imp().request_id.borrow_mut() = new_id;
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
