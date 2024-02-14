use opencv::core::{Mat, MatTraitConst};
use opencv::imgcodecs::imread;
use anyhow::Result;
use crate::device::camera::source::{Source, VideoInfo};

pub struct Image {
    pub cached: Option<Mat>,
    pub path: String,
}

pub struct Video {
    current: u32,
    pub path: String,
}



impl Image {
    pub fn new(path: String) -> Image {
        Image {
            cached: None,
            path,
        }
    }

}

impl Source for Image {
    fn get_info(&mut self) -> Result<VideoInfo> {
        if let Some(cached) = &self.cached {
            return Ok(VideoInfo {
                width: cached.cols(),
                height: cached.rows(),
                fps: 30.0,
            });
        }

        let image = imread(&self.path, 1)?;
        self.cached = Some(image.clone());

        Ok(VideoInfo {
            width: image.cols(),
            height: image.rows(),
            fps: 30.0,
        })
    }

    fn read_to_mat(&mut self) -> Result<Mat> {
        if let Some(cached) = &self.cached {
            return Ok(cached.clone());
        }

        let mat = imread(&self.path, 1)?;
        self.cached = Some(mat.clone());
        Ok(mat)
    }
}

impl Video {
    pub fn new(path: String) -> Video {
        Video {
            current: 0,
            path,
        }
    }
}

impl Source for Video {
    fn get_info(&mut self) -> Result<VideoInfo> {
        unimplemented!()
    }

    fn read_to_mat(&mut self) -> Result<Mat> {
        unimplemented!()
    }
}