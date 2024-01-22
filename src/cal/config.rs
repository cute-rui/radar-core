use opencv::core::{no_array, Point2d, Point2f, Point3d, Vector};
use anyhow::Result;
use ndarray::Array2;
use opencv::calib3d::{rodrigues, solve_pnp, SOLVEPNP_ITERATIVE};
use opencv::imgproc::point_polygon_test;
use crate::cal::pnp::Args;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ObjectType {
    Car,
    Red,
    Blue,
    Red1,
    Red2,
    Red3,
    Red4,
    Red5,
    Blue1,
    Blue2,
    Blue3,
    Blue4,
    Blue5,

    //todo: holding for highland detection adaptor
    Env,
}

impl ObjectType {
    //todo: dirty code
    pub fn is_armor(&self) -> bool {
        self != &ObjectType::Car && self != &ObjectType::Env
    }
}

#[derive(PartialEq)]
pub enum Color {
    Red,
    Blue,
}

pub enum ActionType {
    Attack,
    Defend,
    None,
}

pub struct Container(pub(crate) [Array2<f64>; 4]);
pub struct Config {
/*    pub camera: CameraConfig,
    pub armor: ArmorConfig,
    pub car: CarConfig,
    pub env: EnvConfig,
    pub serial: SerialConfig,
    pub debug: DebugConfig,*/
}



//todo: fetch 3d map args,
pub fn get_point_3d(from_config: bool, color: Color/*, config: Config*/) -> Result<Vector<Point3d>> {
    if !from_config {
        if color == Color::Red {
            return Ok(Vector::from_slice(&[
                Point3d::new(9.47097-0.649, 9.06324-0.433, 0.615),
                Point3d::new(9.47098-0.649, 9.72324-0.433, 0.615),
                Point3d::new(22.13425-0.649,15.48899-0.433, 0.2),
                Point3d::new(25.87255-0.649, 15.48858-0.433, 0.4),
                Point3d::new(24.17254-0.649, 2.10967-0.433, 0.4),
                Point3d::new(10.90243-0.649, 7.60410-0.433, 0.6),
            ]));
        } else {
           return Ok(Vector::from_slice(&[
               Point3d::new(19.86332 - 0.649, 6.83851 - 0.433, 0.615),
               Point3d::new(19.86332 - 0.649, 6.17851 - 0.433, 0.615),
               Point3d::new(7.16102 - 0.649, 0.48926 - 0.433, 0.2),
               Point3d::new(3.42256 - 0.649, 0.58967 - 0.433, 0.4),
               Point3d::new(5.12274 - 0.649, 13.86857 - 0.433, 0.4),
               Point3d::new(18.39284 - 0.649, 8.37415 - 0.433, 0.6),
           ]));
        }
    }

    Err(anyhow::anyhow!("not implemented"))
}

pub fn get_point_2d(threed: Vector<Point3d>, twod_from_input: Vector<Point2d>) -> Result<Vector<Point2d>> {
    let mut result = Vector::new();

    //todo: add length check
    for i in 0..threed.len()-1 {
        println!("3d: {:?}", threed.as_slice()[i]);
        println!("2d: {:?}", twod_from_input.as_slice()[i]);
        result.push(twod_from_input.as_slice()[i]);
    }



    Ok(result)
}



//todo: no height detection
pub fn set_predict_point(mut args: &mut Args) -> Result<()> {
    let threed_point = get_point_3d(false, Color::Red)?;
    let twod_point = get_point_2d(threed_point.clone(), Vector::new())?;

    solve_pnp(&threed_point, &twod_point, &args.intrinsics.K, &args.dist, &mut args.rvec, &mut args.tvec, false, SOLVEPNP_ITERATIVE)?;

    rodrigues(&args.rvec, &mut args.rmat, &mut no_array())?;

    Ok(())
}

//locate_point: (xbar,ybar)
pub fn is_container(parent: &Container, child: &Container) -> Result<bool> {

    let child_average = get_locate_average(child)?;
    let child_point = Point2f::new(child_average[[0, 0]] as f32, child_average[[1, 0]] as f32);


    let parent_input_vec = Vector::from_slice(&[
        Point2f::new(parent.0[0][[0, 0]] as f32, parent.0[0][[1, 0]] as f32),
        Point2f::new(parent.0[1][[0, 0]] as f32, parent.0[1][[1, 0]] as f32),
        Point2f::new(parent.0[2][[0, 0]] as f32, parent.0[2][[1, 0]] as f32),
        Point2f::new(parent.0[3][[0, 0]] as f32, parent.0[3][[1, 0]] as f32),
    ]);

    Ok(point_polygon_test(&parent_input_vec, child_point, false)? >= 0.0)
}



fn get_locate_average(container: &Container) -> Result<Array2<f64>> {
    let average_point = Array2::from_shape_vec((2, 1), vec![container.get_xbar(), container.get_ybar()])?;

    Ok(average_point)
}

impl Container {
    pub fn get_xbar(&self) -> f64 {
        let mut x_vec = vec![];

        for i in self.0.iter() {
            x_vec.push(i[[0, 0]]);
        }

        get_bar(x_vec)
    }

    pub fn get_ybar(&self) -> f64 {
        let mut y_vec = vec![];

        for i in self.0.iter() {
            y_vec.push(i[[0, 1]]);
        }

        get_bar(y_vec)
    }

    fn iter(&self) -> std::slice::Iter<Array2<f64>> {
        self.0.iter()
    }

    pub fn new(point1: [f64; 2], point2: [f64; 2], point3: [f64; 2], point4: [f64; 2]) -> Self {
        let mut container = Container([Array2::zeros((2, 2)), Array2::zeros((2, 2)), Array2::zeros((2, 2)), Array2::zeros((2, 2))]);

        container.0[0] = Array2::from_shape_vec((2, 1), point1.to_vec()).unwrap();
        container.0[1] = Array2::from_shape_vec((2, 1), point2.to_vec()).unwrap();
        container.0[2] = Array2::from_shape_vec((2, 1), point3.to_vec()).unwrap();
        container.0[3] = Array2::from_shape_vec((2, 1), point4.to_vec()).unwrap();

        container
    }
}

fn get_bar(point: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for p in point.iter() {
        sum += p;
    }
    sum / point.len() as f64
}