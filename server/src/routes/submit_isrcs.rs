use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use serde::{Deserialize, Serialize};
use crate::{errors::ApiError, repositories::track_repository::submit_isrcs, AppState, utils::is_valid_publish_token};
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackResponse {
  id: i64,
  name: Option<String>,
  track_name: Option<String>,
  artist_name: Option<String>,
  album_name: Option<String>,
  duration: Option<f64>,
  instrumental: bool,
  plain_lyrics: Option<String>,
  synced_lyrics: Option<String>,
  isrcs: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct SubmitISRCSRequest {
    isrcs: Option<Vec<String>>,
    track_id: i64,
}

pub async fn route(
  headers: HeaderMap,
  State(state): State<Arc<AppState>>,
  Json(payload): Json<SubmitISRCSRequest>,
) -> Result<StatusCode, ApiError> {
  match headers.get("X-Publish-Token") {
    Some(publish_token) => {
      let is_valid = is_valid_publish_token(publish_token.to_str()?, &state.challenge_cache).await;

      if is_valid {
        let isrcs = payload.isrcs.as_deref().unwrap_or(&[]);
        let track_id = payload.track_id;
        let mut conn = state.pool.get()?;
        submit_isrcs(isrcs, track_id, &mut conn)?;

        Ok(StatusCode::CREATED)
      } else {
        Err(ApiError::IncorrectPublishTokenError)
      }
    },
    None => Err(ApiError::IncorrectPublishTokenError)
  }
}
