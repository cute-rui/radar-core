use std::convert::Infallible;



use axum::Router;
use anyhow::Result;
use axum::body::Body;
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::http::{Method, StatusCode};
use axum::response::{Response};
use log::{error};
use crate::share::event::{Action, EventBus, EventObject, EventObjectBuilder};



use tokio_stream::StreamExt;

use tower_http::cors::{Any, CorsLayer};
use crate::calculate;
use crate::common::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub event_bus: EventBus,
    pub config: AppConfig
}

pub fn init_router(event_bus: EventBus, config: AppConfig) -> Router {
    let state = AppState {
        event_bus,
        config
    };

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .layer(cors)
        .route("/render", axum::routing::get(render_handler))
        .route("/ws", axum::routing::get(ws_handler))
        .with_state(state)
}

async fn render_handler(State(app_state): State<AppState>) -> Response {
    let mut rx = app_state.event_bus.clone_rx();
    let (datatx, datarx) = tokio::sync::mpsc::channel(8);

    tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let e = event.clone();
            let d = e.lock().await;
            if d.action == Action::HTTPStreamSend {
                //debug!("sending pic data");
                let raw = ["--frame\r\n".as_bytes().to_vec(), "Content-Type: image/jpeg\r\n\r\n".as_bytes().to_vec(), d.data.to_vec(), "\r\n".as_bytes().to_vec()].concat();
                let data =  bytes::Bytes::from(raw);
                match datatx.send(data).await{
                    Ok(_) => {},
                    Err(e) => {
                        error!("Failed to send data: {}", e);
                        break
                    }
                };
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(datarx)
        .map(Ok::<_, Infallible>);

    let body = Body::from_stream(stream);

    Response::builder().status(StatusCode::OK).header("Content-Type", "multipart/x-mixed-replace; boundary=frame").body(body).unwrap()

}

async fn ws_handler(ws: WebSocketUpgrade, State(app_state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_ws_socket(socket, State(app_state)))
}


async fn handle_ws_socket(mut socket: WebSocket, State(app_state): State<AppState>) {
    let tx = app_state.event_bus.clone_tx();

    let calib_data = calculate::daemon::need_calib_points(app_state.config.clone()).await.unwrap();
    socket.send(Message::Text(calib_data)).await.unwrap();


    while let Some(message) = socket.recv().await {
        let message = match message {
            Ok(message) => message,
            Err(e) => {
                error!("Failed to receive message: {}", e);
                break;
            }
        };

        if message == Message::Ping([0x9].to_vec()) {
            socket.send(Message::Pong([0x9].to_vec())).await;
            continue;
        }

        let event = match parse_message(message) {
            Ok(event) => event,
            Err(e) => {
                error!("Failed to parse message: {}", e);
                continue;
            }
        };

        match tx.send(event.to_event()){
            Ok(_) => {socket.send(Message::Text("ok".to_string())).await;},
             Err(e) => error!("Failed to send event: {}", e)
         };
    }
}

fn parse_message(message: Message) -> Result<EventObject> {
    if let Message::Text(text) = message {
        Ok(EventObject::from_event_builder( EventObjectBuilder::from_text(text)?))
    } else {
        Err(anyhow::anyhow!("Invalid message type"))
    }
}
