//! Security API handlers

use axum::Json;
use serde::Deserialize;
use crate::error::CloudflareResult;
use crate::models::CreateFirewallRule;

#[derive(Deserialize)]
pub struct SetSecurityLevelRequest { pub level: String }

#[derive(Deserialize)]
pub struct ToggleUnderAttackRequest { pub enabled: bool }

#[derive(Deserialize)]
pub struct IpRequest { pub ip: String, pub note: Option<String> }

pub async fn get_security_level() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "level": "medium" } })))
}

pub async fn set_security_level(Json(req): Json<SetSecurityLevelRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "level": req.level } })))
}

pub async fn toggle_under_attack(Json(req): Json<ToggleUnderAttackRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "under_attack": req.enabled } })))
}

pub async fn list_waf_rules() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn list_firewall_rules() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn create_firewall_rule(Json(req): Json<CreateFirewallRule>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": req })))
}

pub async fn list_ip_access_rules() -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": [] })))
}

pub async fn block_ip(Json(req): Json<IpRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "ip": req.ip, "mode": "block" } })))
}

pub async fn allow_ip(Json(req): Json<IpRequest>) -> CloudflareResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({ "success": true, "data": { "ip": req.ip, "mode": "allow" } })))
}
