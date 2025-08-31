use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Serial {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub user_id: u64,
    pub serial_id: i64,
    pub watched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_rating: Option<f64>,
}

impl Serial {
    pub fn new(user_id: u64, serial_id: i64) -> Self {
        Self {
            id: None,
            user_id,
            serial_id,
            watched: false,
            my_rating: None,
        }
    }
}
