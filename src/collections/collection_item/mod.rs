mod imp;

use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct CollectionItem(ObjectSubclass<imp::CollectionItem>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CollectionItem {
    pub fn new(name: &str, id: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("id", id)
            .build()
    }

    pub fn set_collection_name(&self) {
        let name = &*self.imp().name.borrow();
        let label = self.imp().collection_label.clone();

        label.set_label(name);
    }
}
