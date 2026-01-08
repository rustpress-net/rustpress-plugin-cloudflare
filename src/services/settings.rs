//! Settings service for Cloudflare credential management

use crate::error::{CloudflareError, CloudflareResult};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tracing::{debug, info, error};

/// Cloudflare credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareCredentials {
    pub api_token: String,
    pub account_id: String,
    pub zone_id: String,
}

/// Plugin settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginSettings {
    pub cdn_enabled: bool,
    pub cache_level: String,
    pub security_level: String,
    pub ssl_mode: String,
    pub auto_purge_on_update: bool,
    pub development_mode: bool,
}

/// Settings service for managing plugin configuration
#[derive(Clone)]
pub struct SettingsService {
    pool: Pool<Postgres>,
}

impl SettingsService {
    /// Create a new settings service
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Get Cloudflare credentials from database
    pub async fn get_credentials(&self) -> CloudflareResult<Option<CloudflareCredentials>> {
        let api_token = self.get_setting("api_token").await?;
        let account_id = self.get_setting("account_id").await?;
        let zone_id = self.get_setting("zone_id").await?;

        match (api_token, account_id, zone_id) {
            (Some(token), Some(account), Some(zone)) => {
                // Decrypt token if needed
                let token_str = token.as_str().unwrap_or("").trim_matches('"').to_string();
                let account_str = account.as_str().unwrap_or("").trim_matches('"').to_string();
                let zone_str = zone.as_str().unwrap_or("").trim_matches('"').to_string();

                if token_str.is_empty() || account_str.is_empty() || zone_str.is_empty() {
                    return Ok(None);
                }

                Ok(Some(CloudflareCredentials {
                    api_token: token_str,
                    account_id: account_str,
                    zone_id: zone_str,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Save Cloudflare credentials to database
    pub async fn save_credentials(&self, credentials: &CloudflareCredentials) -> CloudflareResult<()> {
        self.set_setting("api_token", &serde_json::json!(credentials.api_token)).await?;
        self.set_setting("account_id", &serde_json::json!(credentials.account_id)).await?;
        self.set_setting("zone_id", &serde_json::json!(credentials.zone_id)).await?;
        info!("Cloudflare credentials saved");
        Ok(())
    }

    /// Delete Cloudflare credentials from database
    pub async fn delete_credentials(&self) -> CloudflareResult<()> {
        self.delete_setting("api_token").await?;
        self.delete_setting("account_id").await?;
        self.delete_setting("zone_id").await?;
        info!("Cloudflare credentials deleted");
        Ok(())
    }

    /// Get plugin settings
    pub async fn get_plugin_settings(&self) -> CloudflareResult<PluginSettings> {
        let cdn_enabled = self.get_setting("cdn_enabled").await?
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let cache_level = self.get_setting("cache_level").await?
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "aggressive".to_string());
        let security_level = self.get_setting("security_level").await?
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "medium".to_string());
        let ssl_mode = self.get_setting("ssl_mode").await?
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "strict".to_string());
        let auto_purge_on_update = self.get_setting("auto_purge_on_update").await?
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let development_mode = self.get_setting("development_mode").await?
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(PluginSettings {
            cdn_enabled,
            cache_level,
            security_level,
            ssl_mode,
            auto_purge_on_update,
            development_mode,
        })
    }

    /// Update plugin settings
    pub async fn update_plugin_settings(&self, settings: &PluginSettings) -> CloudflareResult<()> {
        self.set_setting("cdn_enabled", &serde_json::json!(settings.cdn_enabled)).await?;
        self.set_setting("cache_level", &serde_json::json!(settings.cache_level)).await?;
        self.set_setting("security_level", &serde_json::json!(settings.security_level)).await?;
        self.set_setting("ssl_mode", &serde_json::json!(settings.ssl_mode)).await?;
        self.set_setting("auto_purge_on_update", &serde_json::json!(settings.auto_purge_on_update)).await?;
        self.set_setting("development_mode", &serde_json::json!(settings.development_mode)).await?;
        Ok(())
    }

    /// Get a single setting value
    pub async fn get_setting(&self, key: &str) -> CloudflareResult<Option<serde_json::Value>> {
        let result = sqlx::query_scalar!(
            r#"SELECT value FROM cloudflare_settings WHERE key = $1"#,
            key
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CloudflareError::DatabaseError(e.to_string()))?;

        Ok(result.flatten())
    }

    /// Set a single setting value
    pub async fn set_setting(&self, key: &str, value: &serde_json::Value) -> CloudflareResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO cloudflare_settings (key, value, updated_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (key) DO UPDATE SET value = $2, updated_at = NOW()
            "#,
            key,
            value
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CloudflareError::DatabaseError(e.to_string()))?;

        debug!("Setting '{}' updated", key);
        Ok(())
    }

    /// Delete a setting
    pub async fn delete_setting(&self, key: &str) -> CloudflareResult<()> {
        sqlx::query!(
            r#"DELETE FROM cloudflare_settings WHERE key = $1"#,
            key
        )
        .execute(&self.pool)
        .await
        .map_err(|e| CloudflareError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Check if credentials are configured
    pub async fn has_credentials(&self) -> CloudflareResult<bool> {
        let creds = self.get_credentials().await?;
        Ok(creds.is_some())
    }
}
