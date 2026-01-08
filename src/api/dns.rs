//! DNS API handlers

use axum::{extract::Path, Json};
use crate::error::CloudflareResult;
use crate::models::{CreateDnsRecord, UpdateDnsRecord};

pub async fn list_records() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn get_record(Path(id): Path<String>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "id": id } })))
}

pub async fn create_record(Json(req): Json<CreateDnsRecord>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": req })))
}

pub async fn update_record(
    Path(id): Path<String>,
    Json(req): Json<UpdateDnsRecord>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "id": id } })))
}

pub async fn delete_record(Path(id): Path<String>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "id": id } })))
}

pub async fn export_zone() -> CloudflareResult<String> {
    Ok("; Zone file export".to_string())
}

pub async fn sync_records() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "synced": 0 })))
}
