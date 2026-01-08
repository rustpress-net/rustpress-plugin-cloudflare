//! Page Rules API handlers

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::CloudflareResult;
use crate::services::CloudflareServices;
use crate::models::{CreatePageRule, UpdatePageRule, PageRuleTarget, PageRuleAction, PageRuleConstraint};

/// List page rules
pub async fn list_page_rules(
    State(services): State<Arc<CloudflareServices>>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": []
    })))
}

/// Create page rule
pub async fn create_page_rule(
    State(services): State<Arc<CloudflareServices>>,
    Json(req): Json<CreatePageRule>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": req
    })))
}

/// Update page rule
pub async fn update_page_rule(
    State(services): State<Arc<CloudflareServices>>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePageRule>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "id": id,
            "targets": req.targets,
            "actions": req.actions
        }
    })))
}

/// Delete page rule
pub async fn delete_page_rule(
    State(services): State<Arc<CloudflareServices>>,
    Path(id): Path<String>,
) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "data": { "id": id }
    })))
}
