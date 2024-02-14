pub mod fs;
pub mod source;

/*pub enum camera_type {
    FILE,
    HIKVISION,
    DAHUA,
}

pub struct camera {
    pub K: String,

    pub manufacturer: camera_type,
}

impl camera {
    pub fn new() -> camera {
        camera {
            K: String::from(""),
            manufacturer: camera_type::FILE,
        }
    }

    pub fn read_to_frame(&self) -> Result<Mat> {
        match self.manufacturer {
            camera_type::HIKVISION => {
                log!(Level::Info, "read from hikvision");
            },
            camera_type::DAHUA => {
                log!(Level::Info, "read from dahua");
            },
            _ => {
                return Err(anyhow::anyhow!("Error while reading camera manufacturer"))
            }
        }

        Err(anyhow::anyhow!("Error while reading camera"))
    }

    pub fn is_read_from_file(&self) -> bool {
        match self.manufacturer {
            camera_type::FILE => true,
            _ => false,
        }
    }
}*/