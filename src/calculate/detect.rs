use std::collections::HashMap;



use opencv::core::Point2d;
use anyhow::Result;
use crate::calculate::tools::{ActionType, ObjectType, Container, is_container};
use crate::calculate::pnp::{Args, get_map_location};



//todo:no armor can independently exist without car


pub struct AnyHowDetectedResult {
    env_boxes: Vec<DetectTarget>,
    car_boxes: Vec<DetectTarget>,
    armor_boxes: Vec<DetectTarget>,
}

pub struct DetectTarget {
    action: ActionType,
    confidence: f32,
    category: ObjectType,
    pub location_in_image: Container,

    location_point: [f64; 2],
}

pub struct MappingResult {
    car_map: HashMap<ObjectType, DetectTarget>,
}

impl MappingResult {
    pub fn new() -> Self {
        MappingResult {
            car_map: HashMap::new(),
        }
    }
}


impl AnyHowDetectedResult {

    pub fn mapping(&self, parameter: &Args) -> Result<MappingResult> {
        let mut result = MappingResult::new();
        
        let mut car_map = HashMap::new();

        //highland detection
/*        let mut env_map = HashMap::new();
        for env_box in self.env_boxes.iter() {
            let location = env_box.get_3d_location(parameter);
            env_map.insert(location, env_box);
        }
        result.env_map = env_map;
*/
        
        
        for car_box in self.car_boxes.iter() {
            let mut car = DetectTarget::new();

            car.location_point = get_map_location(parameter, car_box)?;

            for armor_box in self.armor_boxes.iter() {
                if car_box.is_container(armor_box) {
                    if armor_box.confidence < car.confidence {
                       continue
                    }

                    car.category = armor_box.get_catagory();
                    car.confidence = armor_box.confidence;
                }
            }


            car_map.insert(car.category, car);
        }

        result.car_map = car_map;
        Ok(result)
    }

}

impl DetectTarget {

    pub fn new() -> Self {
        DetectTarget {
            action: ActionType::None,
            confidence: 0.0,
            category: ObjectType::Car,
            location_in_image: Container::new([0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]),
            location_point: [0.0, 0.0],
        }
    }
    pub fn is_container(&self, child: &DetectTarget) -> bool {
        is_container(&self.location_in_image, &child.location_in_image).unwrap_or(false)
    }

    pub fn get_image_location(&self) -> Point2d {
        let x = (self.get_x1() + self.get_x2()) / 2.0;
        //todo: accurate y
        let y = self.get_y2() - (self.get_y2() - self.get_y1()) / 9.0;


        Point2d::new(x, y)
    }

    pub fn get_catagory(&self) -> ObjectType {
        self.category
    }

    //todo: check x, y
    fn get_x1(&self) -> f64 {
        self.location_in_image.0[0][[0, 0]]
    }

    fn get_x2(&self) -> f64 {
        //2 points are duplicated
        self.location_in_image.0[3][[0, 1]]
    }

    fn get_y1(&self) -> f64 {
        self.location_in_image.0[0][[0, 0]]
    }

    fn get_y2(&self) -> f64 {
        //2 points are duplicated
        self.location_in_image.0[3][[0, 1]]
    }
}