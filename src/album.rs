use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
#[serde(rename = "for the curious")]
    pub for_the_curious: Option<String>,
    pub current: CurrentRelease,
    pub artist: String,
    pub item_type: String,   // "album", "track", etc.
    pub id: i64,
    pub art_id: i64,
    pub album_release_date: String,
    pub trackinfo: Vec<TrackInfo>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: i64,
    pub track_id: i64,
    pub file: HashMap<String, String>, // "mp3-128" → url, sometimes also "mp3-v0", "flac", etc.
    pub artist: Option<String>,
    pub title: String,
    pub track_num: i32,
    pub title_link: String,
    pub duration: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentRelease {
    pub audit: i32,
    pub title: String,
    pub publish_date: String,
}
