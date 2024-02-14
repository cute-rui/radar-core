
use std::sync::{Arc};

use std::sync::mpsc::{Sender};

use opencv::imgcodecs::{imencode};
use anyhow::Result;
use opencv::core::{MatTraitConstManual, Point, Scalar, Vector};

use std::sync::Mutex;
use bytes::Bytes;

use log::{debug, info};
use opencv::imgproc::HersheyFonts::FONT_HERSHEY_SIMPLEX;
use opencv::imgproc::put_text;


use crate::common::config::{AppConfig};
use crate::{device};
use crate::device::camera::source::{Source, VideoInfo};
use crate::share::event::{Action, EventBus, EventObject, EventSender};

pub async fn calib_rtmp_push(opened: Arc<Mutex<bool>>, config: AppConfig, tx: Sender<Bytes>) -> Result<()> {
    while *(opened.lock().unwrap()) {
        let conf = config.clone();
        let mut source = device::camera::source::new_source_from_config(conf.calib.calib_source_type, conf.calib.calib_source_path, conf.camera);
        let mut img = source.read_to_mat()?;


        put_text(&mut img, &*format!("{}", chrono::Local::now()), Point::new(5, 50), i32::from(FONT_HERSHEY_SIMPLEX), 0.75, Scalar::new(255.0, 255.0, 255.0, 0.0), 2, 0, false)?;

        let data = img.data_bytes()?;
        tx.send(Bytes::from((*data).to_vec()))?;
        /*tokio::time::sleep(std::time::Duration::from_millis(30)).await;*/
    }
    debug!("calib img push end");
    Ok(())
}

pub async fn calib_http_stream_push(opened: Arc<Mutex<bool>>, config: AppConfig, tx: EventSender) -> Result<()> {

    while *(opened.lock().unwrap()) {
        {
            let conf = config.clone();
            let mut source = device::camera::source::new_source_from_config(conf.calib.calib_source_type, conf.calib.calib_source_path, conf.camera);
            let mut img = source.read_to_mat()?;
            //debug!("calib img reading");

            put_text(&mut img, &*format!("{}", chrono::Local::now()), Point::new(5, 50), i32::from(FONT_HERSHEY_SIMPLEX), 0.75, Scalar::new(255.0, 255.0, 255.0, 0.0), 2, 0, false)?;
            let mut output = Vector::<u8>::new();
            imencode(".jpg", &img, &mut output, &Vector::<i32>::new())?;

            let webp = Bytes::from(output.to_vec());

            tx.send(EventObject {
                action: Action::HTTPStreamSend,
                data: webp,
            }.to_event())?;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    debug!("calib img push end");
    Ok(())
}

pub async fn start_calib(config: AppConfig, event_bus: EventBus) -> Result<()> {
    /*let (tx, rx) = mpsc::channel::<Bytes>();*/

    let conf = config.clone();
    let mut info = VideoInfo::default();
    {
        let mut source = device::camera::source::new_source_from_config(conf.calib.calib_source_type, conf.calib.calib_source_path, conf.camera);
        info = source.get_info()?;
    }

    let open = Arc::new(Mutex::new(true));

    let calib = tokio::spawn(calib_http_stream_push(Arc::clone(&open), config.clone(), event_bus.clone().tx));

    let mut rx = event_bus.clone_rx();
    while let event = rx.recv().await? {
        let e = event.lock().await.clone();
        if e.action != Action::EndOfCalib {
            continue
        }

        info!("calib event received: {:?}", e.action);
        let mut op = open.lock().unwrap();
        *op = false;
        break;
    }

    calib.await??;
    //ffmpeg.await??;

    debug!("calib finished");
    Ok(())
}