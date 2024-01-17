use async_channel::{Receiver, Sender};
use lazy_static::lazy_static;

pub enum AppEvent {
    CollectionDeleted(String),
    RequestDeleted(String),
}

lazy_static! {
    pub static ref EVENT_CHANNEL: (Sender<AppEvent>, Receiver<AppEvent>) = {
        let (tx, rx) = async_channel::bounded(1);
        (tx, rx)
    };
}
