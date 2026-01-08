//! Settings API handlers

use axum::Json;
use serde::Deserialize;
use crate::error::CloudflareResult;

#[derive(Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ToggleDevModeRequest {
    pub enabled: bool,
}

pub async fn get_settings() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "cdn_enabled": true,
            "cache_level": "aggressive",
            "security_level": "medium"
        }
    })))
}

pub async fn update_settings(Json(req): Json<UpdateSettingsRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": req.settings })))
}

pub async fn get_zone_settings() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn update_zone_settings(Json(req): Json<serde_json::Value>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": req })))
}

pub async fn toggle_dev_mode(Json(req): Json<ToggleDevModeRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "development_mode": req.enabled } })))
}
