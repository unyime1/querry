mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CollectionWindow(ObjectSubclass<imp::CollectionWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CollectionWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
