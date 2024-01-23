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
        let parsed_url = url.unwrap_or(String::from(""));
        let parsed_http_method = http_method.unwrap_or(String::from(""));

        println!("{}, {}", parsed_http_method, parsed_url);

        Object::builder()
            .property("id", id)
            .property("name", name)
            .property("protocol", protocol)
            .property("httpmethod", parsed_http_method)
            .property("url", parsed_url)
            .build()
    }
}
