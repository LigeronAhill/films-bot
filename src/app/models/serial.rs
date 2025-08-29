use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Serial {
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    user_id: u64,
}
