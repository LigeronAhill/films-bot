use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: u64,
    pub film_id: i64,
    pub watched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_rating: Option<f64>,
}
impl Movie {
    pub fn new(user_id: u64, film_id: i64) -> Self {
        Self {
            id: None,
            user_id,
            film_id,
            watched: false,
            my_rating: None,
        }
    }
}
// Александр Провоторов, [26.08.2025 14:56]
// [Link Text](https://www.example.com)
//
// Александр Провоторов, [26.08.2025 14:56]
// [Google](https://www.google.com)
//
// Александр Провоторов, [26.08.2025 14:58]
// (Google)[https://www.google.com]
//
// Александр Провоторов, [26.08.2025 14:58]
// (ссылка)[https://smth.com]
//
// Александр Провоторов, [26.08.2025 14:58]
// <a href="smth.ru">Сайт</a>
