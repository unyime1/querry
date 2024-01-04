mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CollectionItem(ObjectSubclass<imp::CollectionItem>);
}

impl CollectionItem {
    pub fn new(name: &str, id: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("id", id)
            .build()
    }
}
