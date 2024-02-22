mod imp;

use adw::prelude::BinExt;
use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

use crate::collections::CollectionsWindow;
use crate::requests_view::RequestsView;

glib::wrapper! {
    pub struct RestWindow(ObjectSubclass<imp::RestWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RestWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn embed_children_ui(&self) {
        // Fix collections box and requests_view into UI
        let collections_box = self.imp().collections_box.clone();
        let collection_window = CollectionsWindow::new();
        collections_box.set_child(Some(&collection_window));

        let requests_view_box = self.imp().requests_view_box.clone();
        let requests_view = RequestsView::new();
        requests_view_box.set_child(Some(&requests_view));
    }

    pub fn validate_paned_primary_position(&self) {
        let paned_primary = self.imp().paned_primary.clone();
        paned_primary.connect_position_notify(move |paned| {
            let paned_position = paned.position();
            if paned_position < 300 {
                paned.set_position(300);
            } else if paned_position > 700 {
                paned.set_position(700);
            };
        });
    }

    pub fn validate_paned_secondary_position(&self) {
        let paned_secondary = self.imp().paned_secondary.clone();
        paned_secondary.connect_position_notify(move |paned| {
            let paned_position = paned.position();
            if paned_position < 250 {
                paned.set_position(250);
            }
        });
    }
}
