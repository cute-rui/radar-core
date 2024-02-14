use log::error;
use anyhow::Result;
use crate::calculate::tools::{Color, get_point_3d_in_json_string};
use crate::share::event::{Action, Event, EventBus, EventObject, EventObjectBuilder, EventSender};
use crate::common::config::AppConfig;


//todo: handler processing
pub async fn calculate_daemon(conf: AppConfig, event_bus: EventBus) {
    let tx = event_bus.clone_tx();
    let mut rx = event_bus.clone_rx();

    while let event = rx.recv().await {
        match event {
            Ok(event) => {
                let e = event.lock().await.clone();
                match e.action {
                    _ => {}
                }
            },
            Err(e) => error!("Failed to receive event: {}", e)
        }
    }
}

pub async fn need_calib_points(config: AppConfig) -> Result<String> {
    Ok(get_point_3d_in_json_string(Color::Red)?)
}







