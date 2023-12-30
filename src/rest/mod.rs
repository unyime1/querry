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
}
