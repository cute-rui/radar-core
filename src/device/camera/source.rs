use opencv::core::Mat;
use crate::common::config::{CameraConfig, VideoSource};
use anyhow::Result;
use crate::device::camera::fs::Image;

pub trait Source {
    fn get_info(&mut self) -> Result<VideoInfo>;
    fn read_to_mat(&mut self) -> Result<Mat>;
}

#[derive(Debug, Default)]
pub struct VideoInfo {
    pub width: i32,
    pub height: i32,
    pub fps: f64,
}



pub fn new_source_from_config(source: VideoSource, path: String, camera_config: CameraConfig) -> Box<dyn Source> {
    match source {
       /* VideoSource::Video => {
            Box::new()
        }
        VideoSource::Camera => {
            Box::new()
        }*/
        VideoSource::Image => {
            Box::new(Image::new(path))
        }
        _ => {
            Box::new(Image::new(path))
        }
    }


}