use std::sync::{Arc};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use anyhow::Result;

pub type Event = Arc<Mutex<EventObject>>;

pub type EventSender = tokio::sync::broadcast::Sender<Event>;
pub type EventReceiver = tokio::sync::broadcast::Receiver<Event>;

#[derive(Debug)]
pub struct EventBus {
    pub tx: EventSender,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::broadcast::channel::<Event>(16);

        Self {
            tx,
        }
    }

    pub fn clone_tx(&self) -> EventSender {
        self.tx.clone()
    }

    pub fn clone_rx(&self) -> EventReceiver {
        self.tx.subscribe()
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    Empty,
    WebCalibPointsResult,

    EndOfCalib,
    HTTPStreamSend,
}

#[derive(Debug, Clone)]
pub struct EventObject {
    pub action: Action,
    pub data: Bytes,
}

impl EventObject {
    pub fn new(action: Action, data: Bytes) -> Self {
        Self {
            action,
            data,
        }
    }
    pub fn from_event_builder(builder: EventObjectBuilder) -> Self {
        Self {
            action: builder.action,
            data: Bytes::from(builder.data),
        }
    }
    pub fn to_event(self) -> Event {
        Arc::new(Mutex::new(self.clone()))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventObjectBuilder {
    pub action: Action,
    pub data: String,
}

impl EventObjectBuilder {
    pub fn from_text(text: String) -> Result<Self> {
        Ok(serde_json::from_str(&text)?)
    }

}

pub type FilterFn = fn(Action) -> bool;

/*pub fn new_filter(actions: Vec<Action>) -> FilterFn {
    return move |action| {
        actions.contains(&action)
    }
}*/

/*pub async fn subscribe(mut rx: EventReceiver, filter: FilterFn) -> Result<EventObject> {
    while let Ok(i) = rx.recv().await {
        let e = i.clone();
        if filter(e.lock().await.action) {
            return Ok(i.lock().await.clone());
        }
    }

    Err(anyhow::anyhow!("Failed to receive event"))
}*/

pub fn new_event(action: Action, data: Bytes) -> Event {
    Arc::new(Mutex::new(EventObject {
        action,
        data,
    }))
}

pub fn new_test_event() -> Event {
    Arc::new(Mutex::new(EventObject {
        action: Action::Empty,
        data: Bytes::from("test".to_string()),
    }))
}