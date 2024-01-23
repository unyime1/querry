mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;

use super::request_item::RequestItem;
use crate::utils::crud::requests::{HTTPMethods, ProcolTypes};

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
        let protocol = self.imp().protocol.get();
        let http_method = self.imp().httpmethod.get();

        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind name.
        let name_binding = request_item
            .bind_property("name", &name, "label")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(name_binding);

        // Bind protocol
        let protocol_binding = request_item
            .bind_property("protocol", &protocol, "label")
            .sync_create()
            .build();
        bindings.push(protocol_binding);

        // Bind http_method
        let http_method_binding = request_item
            .bind_property("httpmethod", &http_method, "label")
            .sync_create()
            .build();
        bindings.push(http_method_binding);
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

    /// Define display settings for request box.
    pub fn setup_display(&self) {
        let protocol = self.imp().protocol.clone();
        let request_icon_box = self.imp().request_icon_box.clone();
        let http_method = self.imp().httpmethod.clone();

        // Define which icon box is displayed.
        if &protocol.label() == &ProcolTypes::Http.to_string() {
            http_method.set_visible(true);
            protocol.set_visible(false);
            self.define_http_method_box_background(&http_method.label());
        } else {
            http_method.set_visible(false);
            protocol.set_visible(true);
            request_icon_box.set_css_classes(&vec!["color-blue"])
        }
    }

    pub fn define_http_method_box_background(&self, http_method: &str) {
        let request_icon_box = self.imp().request_icon_box.clone();

        if http_method == HTTPMethods::Post.to_string() {
            request_icon_box.set_css_classes(&vec!["color-green"])
        } else if http_method == HTTPMethods::Get.to_string() {
            request_icon_box.set_css_classes(&vec!["color-blue"])
        } else if http_method == HTTPMethods::Put.to_string() {
            request_icon_box.set_css_classes(&vec!["color-orange"])
        } else if http_method == HTTPMethods::Delete.to_string() {
            request_icon_box.set_css_classes(&vec!["color-red"])
        }
    }
}
