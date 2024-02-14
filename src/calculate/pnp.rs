use opencv::core::{Mat, MatTraitConst};
use anyhow::{anyhow, Result};
use crate::calculate::detect::DetectTarget;

//todo: init once
pub struct Args {
    pub intrinsics: Intrinsics,

    pub rvec: Mat, //Rotation Vector
    pub rmat: Mat, //Rotation Matrix
    pub tvec: Mat,
    pub dist: Mat, //Distortion Coefficients

}

pub struct Intrinsics {
    pub K: Mat,
}

impl Args {
    pub fn get_R_args(&self) -> Result<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> {
        Ok((
         *self.rmat.at_2d::<f64>(0, 0)?, *self.rmat.at_2d::<f64>(0, 1)?, *self.rmat.at_2d::<f64>(0, 2)?,
            *self.rmat.at_2d::<f64>(1, 0)?, *self.rmat.at_2d::<f64>(1, 1)?, *self.rmat.at_2d::<f64>(1, 2)?,
            *self.rmat.at_2d::<f64>(2, 0)?, *self.rmat.at_2d::<f64>(2, 1)?, *self.rmat.at_2d::<f64>(2, 2)?
        ))
    }

    pub fn get_t_args(&self) -> Result<(f64, f64, f64)> {
        Ok((
            *self.tvec.at_2d::<f64>(0, 0)?, *self.tvec.at_2d::<f64>(1, 0)?, *self.tvec.at_2d::<f64>(2, 0)?
        ))
    }
}

impl Intrinsics {
    pub fn get_k_args(&self) -> Result<(f64, f64, f64, f64)> {
        Ok((
         *self.K.at_2d::<f64>(0, 0)?, *self.K.at_2d::<f64>(1, 1)?,
         *self.K.at_2d::<f64>(0, 2)?, *self.K.at_2d::<f64>(1, 2)?
        ))
    }
}

pub fn get_map_location(raw_arg: &Args, target: &DetectTarget) -> Result<[f64; 2]> {
    let args = raw_arg;

    let height = 0.0;
    let (fx, fy, cx, cy) = args.intrinsics.get_k_args()?;
    let (a, b, c, d, e, f, g, h, i) = args.get_R_args()?;
    let (tx, ty, tz) = args.get_t_args()?;

    let point = target.get_image_location();
    let (img_x, img_y) = (point.x, point.y);

    let A = (img_x - cx) / fx;
    let B = (img_y - cy) / fy;

    let Z = (a - A * g) * (e - B * h) - (d - B * g) * (b - A * h);

    if Z != 0 as f64 {
        let X = (((A * i - c) * height + (A * tz - tx)) * (e - B * h) - ((B * i - f) * height + (B * tz - ty)) * (b - A * h)) / Z;
        let Y = ((a - A * g) * ((B * i - f) * height + (B * tz - ty)) - (d - B * g) * ((A * i - c) * height +(A * tz - tx))) / Z;
        Ok([X, Y])
    } else {
        Err(anyhow!("Z is zero"))
    }
}
