use serde::{Deserialize, Serialize};
use anyhow::{Result};
use config::{Config, File};
use std::fs;
use std::io::Write;
use toml;


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub rtmp: RtmpConfig,
    pub ws: WSConfig,


    pub camera: CameraConfig,

    pub log: LogConfig,

    pub calib: CalibConfig,

    pub detect: DetectConfig,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RtmpConfig {
    pub enabled: bool,
    pub addr: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WSConfig {
    pub enabled: bool,
    pub addr: String,
    pub port: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraManufacturer {
    Hikvision,
    Dahua,
    Other,
}

impl Default for CameraManufacturer {
    fn default() -> Self {
        CameraManufacturer::Other
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub manufacturer: CameraManufacturer,
    pub model: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub path: String,
    pub show: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoSource {
    Image,
    Video,
    Camera,
}

impl Default for VideoSource {
    fn default() -> Self {
        VideoSource::Image
    }
}

impl VideoSource {
    fn default() -> Self {
        VideoSource::Image
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CalibConfig {
    pub calib_source_type: VideoSource,
    pub calib_source_path: String,

    pub calib_points: Vec<CalibPoint>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CalibPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DetectConfig {
    pub detect_source_type: VideoSource,
    pub model: String,
    pub adapter: String,
}



impl AppConfig {
    pub fn new() -> Result<Self> {
        let mut config = AppConfig::default();

        config.rtmp.enabled = true;
        config.rtmp.addr = "0.0.0.0".parse()?;

        config.ws.enabled = true;
        config.ws.addr = "0.0.0.0".parse()?;
        config.ws.port = "8080".parse()?;

        config.camera.manufacturer = CameraManufacturer::Other;
        config.camera.model = "".parse()?;

        config.detect.detect_source_type = VideoSource::Image;
        config.detect.model = "yolov8m".parse()?;
        config.detect.adapter = "openvino".parse()?;

        config.log.level = "info".parse()?;
        config.log.path = "log.txt".parse()?;
        config.log.show = true;

        config.calib.calib_source_type = VideoSource::Image;
        config.calib.calib_source_path = "calib.jpg".parse()?;

        config.calib.calib_points = vec![
            CalibPoint { x: 0.0, y: 0.0, z: 0.0 },
            CalibPoint { x: 0.0, y: 0.0, z: 0.0 },
            CalibPoint { x: 0.0, y: 0.0, z: 0.0 },
            CalibPoint { x: 0.0, y: 0.0, z: 0.0 },
            CalibPoint { x: 0.0, y: 0.0, z: 0.0 },
            CalibPoint { x: 0.0, y: 0.0, z: 0.0 },
        ];

        Ok(config)
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let config_ = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        let config: AppConfig = config_.try_deserialize()?;

        Ok(config)
    }
}

pub fn read_or_create_config(path: &str) -> Result<AppConfig> {
    let config = match AppConfig::load_from_file(path) {
        Ok(c) => c,
        Err(_e) => {
            let config = AppConfig::new()?;
            let context = toml::to_string(&config)?;
            create_config(context, path)?;
            config
        }
    };

    Ok(config)
}

fn create_config(context: String, path: &str) -> Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(context.as_bytes())?;
    Ok(())
}

