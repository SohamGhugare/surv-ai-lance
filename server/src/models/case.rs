use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// case model
#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub id: u16,
    pub camera_id: String,
    pub footage_url: String,
    pub camera_location: Location,
    pub timestamp: DateTime<Utc>,
}

// location model for cctv location
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

// create case model
#[derive(Serialize, Deserialize)]
pub struct CreateCase {
    pub camera_id: String,
    pub footage_url: String,
    pub camera_location: Location,
}

// methods for case
impl Case {
    pub fn new(case: CreateCase) -> Self {
        // generate random id
        let id = rand::random::<u16>();
        Case {
            id,
            camera_id: case.camera_id,
            footage_url: case.footage_url,
            camera_location: case.camera_location,
            timestamp: Utc::now(),
        }
    }
}
