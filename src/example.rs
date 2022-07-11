use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

enum TgTypes {
    Location(Location),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "longitude")]
    longitude: f64,
    #[serde(rename = "latitude")]
    latitude: f64,
    #[serde(rename = "horizontal_accuracy")]
    horizontal_accuracy: Option<f64>,
    #[serde(rename = "live_period")]
    live_period: Option<i64>,
    #[serde(rename = "heading")]
    heading: Option<i64>,
    #[serde(rename = "proximity_alert_radius")]
    proximity_alert_radius: Option<i64>,
}
