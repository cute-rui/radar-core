extern crate rtmp;
extern crate opencv;

use std::io::Write;
use rtmp::{
    relay::{pull_client::PullClient, push_client::PushClient},
    rtmp::RtmpServer,
};
use opencv::imgcodecs::{imread, IMREAD_COLOR};
use opencv::prelude::*;
use opencv::videoio;
use {anyhow::Result, streamhub::StreamsHub};
use std::process::{Command, Stdio};
//todo: intergrate
#[tokio::main]
async fn main() -> Result<()> {
    start_single_server();

    let mut cap = imread("calib.jpg", IMREAD_COLOR).unwrap();

    let mut output = Command::new("ffmpeg");
    output
        .arg("-y")
        .arg("-f")
        .arg("rawvideo")
        .arg("-vcodec")
        .arg("rawvideo")
        .arg("-pix_fmt")
        .arg("bgr24")
        .arg("-s")
        .arg("1440x1080")
        .arg("-r")
        .arg("30")
        .arg("-i")
        .arg("-") // 从stdin读取帧
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("ultrafast")
        .arg("-f")
        .arg("flv")
        .arg("rtmp://localhost:1935"); // 替换为实际的流媒体服务器URL


    output.stdin(Stdio::piped());

    let child = output.spawn();

    // 检查子进程是否成功启动
    let mut child = match child {
        Ok(child) => child,
        Err(err) => {
            println!("Failed to start ffmpeg: {}", err);
            return Err(err.into());
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        // 在这里可以写入数据到子进程的stdin管道
        loop {
            stdin.write_all(cap.data_bytes()?)?
        }
    };
    Ok(())
}

fn start_single_server() {
    let mut stream_hub = StreamsHub::new(None);
    let sender = stream_hub.get_hub_event_sender();

    let listen_port = 1935;
    let address = format!("0.0.0.0:{port}", port = listen_port);

    let mut rtmp_server = RtmpServer::new(address, sender, 1);
    tokio::spawn(async move {
        if let Err(err) = rtmp_server.run().await {
            log::error!("rtmp server error: {}\n", err);
        }
    });

    tokio::spawn(async move { stream_hub.run().await });
}
