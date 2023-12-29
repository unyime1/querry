mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct RestWindow(ObjectSubclass<imp::RestWindow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RestWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
