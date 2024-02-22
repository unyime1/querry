mod imp;

use crate::utils::crud::requests::{HTTPMethods, ProtocolTypes};
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
        let parsed_http_method = &http_method.unwrap_or(String::from(""));
        let icon = compute_request_icon(&protocol, &parsed_http_method);

        Object::builder()
            .property("id", id)
            .property("name", name)
            .property("protocol", protocol)
            .property("httpmethod", parsed_http_method)
            .property("url", parsed_url)
            .property("icon", icon)
            .build()
    }
}

/// Compute the right icon for request box.
pub fn compute_request_icon(protocol: &String, http_method: &String) -> String {
    let parsed_protocol = match ProtocolTypes::from_string(&protocol) {
        Some(data) => data,
        None => ProtocolTypes::Http,
    };

    if parsed_protocol == ProtocolTypes::Http {
        let parsed_http_method = match HTTPMethods::from_string(&http_method) {
            Some(data) => data,
            None => HTTPMethods::Get,
        };

        match parsed_http_method {
            HTTPMethods::Post => "resources/icons/post.png".to_string(),
            HTTPMethods::Get => "resources/icons/get.png".to_string(),
            HTTPMethods::Put => "resources/icons/put.png".to_string(),
            HTTPMethods::Delete => "resources/icons/delete.png".to_string(),
        }
    } else {
        "resources/icons/get.png".to_string()
    }
}
