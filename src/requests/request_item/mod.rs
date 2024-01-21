mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct RequestItem(ObjectSubclass<imp::RequestItem>);
}

impl RequestItem {
    pub fn new(name: &str, id: &str, url: &str, protocol: &str, http_method: &str) -> Self {
        Object::builder()
            .property("id", id)
            .property("name", name)
            .property("protocol", protocol)
            .property("http_method", http_method)
            .build()
    }
}
