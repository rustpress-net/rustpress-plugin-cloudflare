//! Workers API handlers

use axum::{extract::Path, Json};
use serde::Deserialize;
use crate::error::CloudflareResult;

#[derive(Deserialize)]
pub struct DeployWorkerRequest { pub name: String, pub script: String }

#[derive(Deserialize)]
pub struct CreateRouteRequest { pub pattern: String, pub script: String }

#[derive(Deserialize)]
pub struct CreateKvNamespaceRequest { pub title: String }

#[derive(Deserialize)]
pub struct SetKvValueRequest { pub value: String }

pub async fn list_workers() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn get_worker(Path(name): Path<String>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "name": name } })))
}

pub async fn deploy_worker(Json(req): Json<DeployWorkerRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "name": req.name } })))
}

pub async fn delete_worker(Path(name): Path<String>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "name": name } })))
}

pub async fn list_routes() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn create_route(Json(req): Json<CreateRouteRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "pattern": req.pattern } })))
}

pub async fn list_kv_namespaces() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn create_kv_namespace(Json(req): Json<CreateKvNamespaceRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "title": req.title } })))
}

pub async fn list_kv_keys(Path(namespace): Path<String>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn get_kv_value(Path((namespace, key)): Path<(String, String)>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": null })))
}

pub async fn set_kv_value(
    Path((namespace, key)): Path<(String, String)>,
    Json(req): Json<SetKvValueRequest>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn delete_kv_value(Path((namespace, key)): Path<(String, String)>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true })))
}
