//! Stream API handlers

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::CloudflareResult;
use crate::services::CloudflareServices;

#[derive(Debug, Deserialize)]
pub struct CreateLiveInputRequest {
    pub name: Option<String>,
    pub recording_mode: Option<String>,
}

/// List Stream videos
pub async fn list_videos(
    State(services): State<Arc<CloudflareServices>>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": []
    })))
}

/// Get Stream video details
pub async fn get_video(
    State(services): State<Arc<CloudflareServices>>,
    Path(id): Path<String>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "uid": id
        }
    })))
}

/// Delete Stream video
pub async fn delete_video(
    State(services): State<Arc<CloudflareServices>>,
    Path(id): Path<String>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": { "uid": id }
    })))
}

/// List live inputs
pub async fn list_live_inputs(
    State(services): State<Arc<CloudflareServices>>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": []
    })))
}

/// Create live input
pub async fn create_live_input(
    State(services): State<Arc<CloudflareServices>>,
    Json(req): Json<CreateLiveInputRequest>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "name": req.name.unwrap_or_else(|| "Live Stream".to_string()),
            "recording_mode": req.recording_mode.unwrap_or_else(|| "off".to_string())
        }
    })))
}

/// Delete live input
pub async fn delete_live_input(
    State(services): State<Arc<CloudflareServices>>,
    Path(id): Path<String>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": { "uid": id }
    })))
}
