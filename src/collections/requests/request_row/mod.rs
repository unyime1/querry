mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, EventControllerMotion, EventSequenceState, GestureClick};

use super::request_item::RequestItem;
use crate::utils::{
    messaging::{AppEvent, EVENT_CHANNEL},
    tokio_runtime::runtime,
};

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

    pub fn get_request_id(&self) -> String {
        let request_id = self.imp().request_id.clone();
        request_id.label().to_string()
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

    /// Emit ViewRequestItem message when name is clicked.
    pub fn listen_to_label_click(&self) {
        let gesture_name = GestureClick::new();
        let request_id = self.get_request_id();

        // Listen to click signal.
        gesture_name.connect_released(move |gesture, _, _, _| {
            gesture.set_state(EventSequenceState::Claimed);
            let request_id_copy = request_id.clone();

            // Send a View request message.
            // Message is consumed on RequestsView widget.
            runtime().spawn(async move {
                EVENT_CHANNEL
                    .0
                    .send(AppEvent::ViewRequestItem(request_id_copy))
                    .await
                    .expect("Channel should be open");
            });
        });

        self.add_controller(gesture_name);
    }

    pub fn bind(&self, request_item: &RequestItem) {
        // Get state
        let name = self.imp().name.get();
        let request_logo = self.imp().request_icon.get();
        let request_id = self.imp().request_id.get();

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

        // Bind id
        let id_binding = request_item
            .bind_property("id", &request_id, "label")
            .sync_create()
            .build();
        bindings.push(id_binding);

        // Start listening to click signals.
        // Moved here because for some reason request_id is unavailable
        // When called at ObjectImpl.
        self.listen_to_label_click();
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
