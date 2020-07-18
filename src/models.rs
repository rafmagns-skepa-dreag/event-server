use serde::Deserialize;

#[derive(Deserialize)]
pub struct Event {
    pub event_name: String,
    pub user: String,
    pub magnitude: Option<f32>,
}
