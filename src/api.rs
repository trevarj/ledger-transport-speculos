use std::fmt::Display;

use reqwest::header::{HeaderValue, ACCEPT};
use serde::Deserialize;
use serde_json::json;

use crate::TransportSpeculosHttp;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Both,
}

#[derive(Debug, Deserialize)]
pub struct EventsResponse {
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Event {
    pub text: String,
    pub x: u32,
    pub y: u32,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let button = match self {
            Button::Left => "left",
            Button::Right => "right",
            Button::Both => "both",
        };
        write!(f, "{button}")
    }
}

impl TransportSpeculosHttp {
    /// Press and release specified button or both
    pub async fn button(&self, button: Button) -> Result<(), ApiError> {
        let url = format!("{}/button/{button}", self.url);
        self.client
            .post(url)
            .json(&json! ({"action": "press-and-release"}))
            .send()
            .await?;
        Ok(())
    }

    /// Get events produced by the app
    pub async fn events(&self, current_screen_only: bool) -> Result<EventsResponse, ApiError> {
        let url = format!("{}/events", self.url);
        let req = self
            .client
            .get(url)
            .header(ACCEPT, HeaderValue::from_static("application/json"));
        let mut query = vec![];
        if current_screen_only {
            query.push(("currentscreenonly", "true"));
        }
        query.push(("stream", "false"));

        Ok(req
            .query(&query)
            .send()
            .await?
            .json::<EventsResponse>()
            .await?)
    }

    /// Resets device event list
    pub async fn reset_events(&self) -> Result<(), ApiError> {
        let url = format!("{}/events", self.url);
        self.client.delete(url).send().await?;
        Ok(())
    }

    /// Takes a screenshot of current screen
    pub async fn screenshot(&self) {}
}
