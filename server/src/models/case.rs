use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// case model
#[derive(Serialize, Deserialize)]
pub struct Case {
    pub id: u16,
    pub camera_id: String,
    pub footage_url: String,
    pub camera_location: Location,
    pub timestamp: DateTime<Utc>,
}

// location model for cctv location
#[derive(Serialize, Deserialize)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}
