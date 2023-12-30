mod imp;

use adw::prelude::BinExt;
use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

use crate::collection::CollectionWindow;

glib::wrapper! {
    pub struct RestWindow(ObjectSubclass<imp::RestWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RestWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn fix_collections_ui(&self) {
        let collections_box = self.imp().collections_box.clone();
        let collection_window = CollectionWindow::new();
        collections_box.set_child(Some(&collection_window));
    }

    pub fn validate_paned_primary_position(&self) {
        let paned_primary = self.imp().paned_primary.clone();
        paned_primary.connect_position_notify(move |paned| {
            let paned_position = paned.position();
            if paned_position < 350 {
                paned.set_position(350);
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
