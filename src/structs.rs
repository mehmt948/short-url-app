use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShortUrl {
    pub url: String,
    pub short_id: String,
    pub timestamp: bson::DateTime,
}

#[derive(Deserialize)]
pub struct NewShortUrlBody {
    pub url: String,
}