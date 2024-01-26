mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, EventControllerMotion};

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

    /// Compute visibility of collection menu on hover.
    pub fn process_hover(&self) {
        // Get widgets.
        let request_menu = self.imp().request_menu.clone();
        let request_menu_clone = request_menu.clone();

        // Make menu button invisible by default.
        request_menu.set_opacity(0.0);

        // Make visible on hover enter and invisible on hover leave.
        let enter_handler = EventControllerMotion::new();
        enter_handler.connect_enter(move |_, _, _| {
            request_menu_clone.set_opacity(1.0);
        });
        enter_handler.connect_leave(move |_| {
            request_menu.set_opacity(0.0);
        });

        self.add_controller(enter_handler);
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
