mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct RequestItem(ObjectSubclass<imp::RequestItem>);
}

impl RequestItem {
    pub fn new(
        name: String,
        id: String,
        url: Option<String>,
        protocol: String,
        http_method: Option<String>,
    ) -> Self {
        Object::builder()
            .property("id", id)
            .property("name", name)
            .property("protocol", protocol)
            .property("http_method", http_method)
            .property("url", url)
            .build()
    }
}
