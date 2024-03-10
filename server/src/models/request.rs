use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::case::Location;

// request model
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastRequest {
    pub id: u16,
    pub camera_id: String,
    pub footage_url: String,
    pub camera_location: Location,
    pub timestamp: DateTime<Utc>,
}

// write a display implementation for BroadcastRequest
impl Display for BroadcastRequest {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"id\": {}, \"camera_id\": {}, \"footage_url\": \"{}\", \"camera_location\": \"{:?}\", \"timestamp\": \"{}\"}}",
            self.id, self.camera_id, self.footage_url, self.camera_location, self.timestamp
        )
    }
}
