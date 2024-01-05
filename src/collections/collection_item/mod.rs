mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CollectionItem(ObjectSubclass<imp::CollectionItem>);
}

impl CollectionItem {
    pub fn new(name: &str, id: &str, icon: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("id", id)
            .property("icon", icon)
            .build()
    }

    pub fn update_icon(&self, new_icon: &str) {
        self.set_icon(new_icon);
    }
}
