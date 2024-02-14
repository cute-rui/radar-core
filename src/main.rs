use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time::Duration;
use anyhow::{Error, Result};
use axum::Router;
use axum::routing::{get, post};
use env_logger::Env;
use log::{debug, error, log_enabled, info, Level};
use opencv::core::Mat;
use opencv::core;
use opencv::photo::texture_flattening;
use opencv::videoio::{VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst};
use tokio::join;
use crate::calculate::daemon::calculate_daemon;
use crate::common::config::read_or_create_config;
use crate::common::image::test;
use crate::calculate::calib::{start_calib};
use crate::share::event::{EventBus, EventObject, new_test_event};
use crate::srv::ws::init_router;

mod calculate;
mod device;

mod srv;

mod share;
mod common;

mod inference;
const CONFIG_FILE: &str = "config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "radar_core=debug");
    env_logger::init_from_env(env);

    let config = read_or_create_config(CONFIG_FILE)?;

    info!("{:?}", config);

    let event_bus = EventBus::new();

    let cal = tokio::spawn(calculate_daemon(config.clone(), event_bus.clone()));

    let calib = tokio::spawn(start_calib(config.clone(), event_bus.clone()));


    let router = init_router(event_bus.clone(), config.clone());
    let bind = format!("{}:{}", config.ws.addr, config.ws.port);



   let ws = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    });

    join!(ws, calib, cal);

    Ok(())
}


