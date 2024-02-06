use lazy_static::lazy_static;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Clone, Debug)]
pub enum AppEvent {
    CollectionDeleted(String),
    RequestDeleted(String),
    ViewRequestItem(String),
    RenameRequestItem(String, String, String), // new_name, request_id, collection_id
}

lazy_static! {
    pub static ref EVENT_CHANNEL: (Sender<AppEvent>, Receiver<AppEvent>) = {
        let (tx, rx) = broadcast::channel(100);
        (tx, rx)
    };
}
