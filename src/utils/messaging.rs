use crate::utils::crud::requests::HTTPMethods;
use lazy_static::lazy_static;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Clone, Debug)]
pub enum AppEvent {
    CollectionDeleted(String),
    UpdateHttpMethod(HTTPMethods, String, String), // new_http_method, request_id, collection_id
    ViewRequestItem(String),
    RenameRequestItem(String, String, String), // new_name, request_id, collection_id
}

lazy_static! {
    pub static ref EVENT_CHANNEL: (Sender<AppEvent>, Receiver<AppEvent>) = {
        let (tx, rx) = broadcast::channel(100);
        (tx, rx)
    };
}
