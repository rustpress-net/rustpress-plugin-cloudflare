//! # RustCloudflare
//!
//! Seamless Cloudflare integration for RustPress - CDN, DNS, Security, Workers, R2, KV, D1, and more.
//!
//! This plugin provides comprehensive Cloudflare management capabilities directly from your
//! RustPress admin dashboard, including:
//!
//! - **CDN Management**: Cache control, purging, and optimization
//! - **DNS Management**: Full DNS record CRUD operations
//! - **Security**: WAF rules, firewall, bot management, DDoS protection
//! - **SSL/TLS**: Certificate management and configuration
//! - **Workers**: Deploy and manage Cloudflare Workers
//! - **KV Storage**: Workers KV namespace and key management
//! - **R2 Storage**: S3-compatible object storage for media
//! - **D1 Database**: Edge SQL database integration
//! - **Stream**: Video hosting and live streaming
//! - **Analytics**: Traffic, performance, and security analytics

pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod middleware;
pub mod models;
pub mod services;
pub mod workers;

use async_trait::async_trait;
use rustpress_core::{
    plugin::{Plugin, PluginInfo, PluginState},
    AppContext,
};
use rustpress_plugins::lifecycle::{
    ActivationContext, DeactivationContext, HookError, InitContext, LifecycleHook,
    LoadContext, ShutdownContext, UpgradeContext,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::client::CloudflareClient;
use crate::config::CloudflareConfig;
use crate::services::CloudflareServices;

/// Current plugin version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin identifier
pub const PLUGIN_ID: &str = "rustcloudflare";

/// Main RustCloudflare plugin struct
pub struct RustCloudflarePlugin {
    info: PluginInfo,
    state: RwLock<PluginState>,
    config: RwLock<Option<CloudflareConfig>>,
    client: RwLock<Option<Arc<CloudflareClient>>>,
    services: RwLock<Option<Arc<CloudflareServices>>>,
}

impl RustCloudflarePlugin {
    /// Create a new instance of the plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: PLUGIN_ID.to_string(),
                name: "RustCloudflare".to_string(),
                version: semver::Version::parse(VERSION).unwrap_or_else(|_| {
                    semver::Version::new(1, 0, 0)
                }),
                description: "Seamless Cloudflare integration for RustPress".to_string(),
                author: "RustPress Team".to_string(),
                author_url: Some("https://rustpress.io".to_string()),
                homepage: Some("https://rustpress.io/plugins/rustcloudflare".to_string()),
                license: "MIT".to_string(),
                dependencies: vec![],
                min_rustpress_version: Some(semver::Version::new(1, 0, 0)),
                tags: vec![
                    "cloudflare".to_string(),
                    "cdn".to_string(),
                    "security".to_string(),
                    "performance".to_string(),
                    "dns".to_string(),
                    "workers".to_string(),
                    "edge".to_string(),
                ],
            },
            state: RwLock::new(PluginState::Inactive),
            config: RwLock::new(None),
            client: RwLock::new(None),
            services: RwLock::new(None),
        }
    }

    /// Initialize the Cloudflare client with configuration
    async fn init_client(&self, ctx: &AppContext) -> Result<(), error::CloudflareError> {
        // Load configuration from settings
        let config = CloudflareConfig::from_context(ctx).await?;

        // Create the Cloudflare API client
        let client = Arc::new(CloudflareClient::new(&config)?);

        // Verify connection
        client.verify_connection().await?;

        // Create services layer
        let services = Arc::new(CloudflareServices::new(
            Arc::clone(&client),
            ctx.db_pool().clone(),
        ));

        // Store in plugin state
        *self.config.write().await = Some(config);
        *self.client.write().await = Some(client);
        *self.services.write().await = Some(services);

        Ok(())
    }

    /// Get the Cloudflare client
    pub async fn client(&self) -> Option<Arc<CloudflareClient>> {
        self.client.read().await.clone()
    }

    /// Get the services layer
    pub async fn services(&self) -> Option<Arc<CloudflareServices>> {
        self.services.read().await.clone()
    }

    /// Get the current configuration
    pub async fn config(&self) -> Option<CloudflareConfig> {
        self.config.read().await.clone()
    }
}

impl Default for RustCloudflarePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for RustCloudflarePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    async fn activate(&self, ctx: &AppContext) -> anyhow::Result<()> {
        info!("Activating RustCloudflare plugin v{}", VERSION);

        *self.state.write().await = PluginState::Activating;

        // Run database migrations
        self.run_migrations(ctx).await?;

        // Initialize the client
        match self.init_client(ctx).await {
            Ok(_) => {
                info!("Cloudflare client initialized successfully");
            }
            Err(e) => {
                warn!("Cloudflare client initialization deferred: {}", e);
                // Don't fail activation - user may need to configure settings first
            }
        }

        // Register API routes
        api::register_routes(ctx).await?;

        // Register admin pages
        self.register_admin_pages(ctx).await?;

        // Register hooks
        self.register_hooks(ctx).await?;

        *self.state.write().await = PluginState::Active;
        info!("RustCloudflare plugin activated successfully");

        Ok(())
    }

    async fn deactivate(&self, ctx: &AppContext) -> anyhow::Result<()> {
        info!("Deactivating RustCloudflare plugin");

        *self.state.write().await = PluginState::Deactivating;

        // Cleanup resources
        *self.client.write().await = None;
        *self.services.write().await = None;
        *self.config.write().await = None;

        // Unregister API routes
        api::unregister_routes(ctx).await?;

        *self.state.write().await = PluginState::Inactive;
        info!("RustCloudflare plugin deactivated");

        Ok(())
    }

    async fn on_startup(&self, ctx: &AppContext) -> anyhow::Result<()> {
        info!("RustCloudflare plugin starting up");

        // Try to initialize client if not already done
        if self.client.read().await.is_none() {
            if let Err(e) = self.init_client(ctx).await {
                warn!("Cloudflare client not initialized: {}", e);
            }
        }

        // Start background tasks
        self.start_background_tasks(ctx).await?;

        Ok(())
    }

    async fn on_shutdown(&self, _ctx: &AppContext) -> anyhow::Result<()> {
        info!("RustCloudflare plugin shutting down");

        // Stop background tasks
        self.stop_background_tasks().await?;

        Ok(())
    }

    fn is_compatible(&self) -> bool {
        true
    }

    fn config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "api_token": {
                    "type": "string",
                    "title": "API Token",
                    "description": "Cloudflare API token"
                },
                "account_id": {
                    "type": "string",
                    "title": "Account ID"
                },
                "zone_id": {
                    "type": "string",
                    "title": "Zone ID"
                }
            },
            "required": ["api_token", "account_id", "zone_id"]
        }))
    }

    fn state(&self) -> PluginState {
        // This is a sync method, so we use try_read
        self.state.try_read().map(|s| s.clone()).unwrap_or(PluginState::Inactive)
    }
}

impl RustCloudflarePlugin {
    /// Run database migrations
    async fn run_migrations(&self, ctx: &AppContext) -> anyhow::Result<()> {
        let pool = ctx.db_pool();

        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .map_err(|e| {
                error!("Migration failed: {}", e);
                anyhow::anyhow!("Database migration failed: {}", e)
            })?;

        info!("Database migrations completed");
        Ok(())
    }

    /// Register admin pages
    async fn register_admin_pages(&self, ctx: &AppContext) -> anyhow::Result<()> {
        // Admin pages are registered via plugin.toml manifest
        // This method can be used for dynamic page registration
        Ok(())
    }

    /// Register hooks for RustPress events
    async fn register_hooks(&self, ctx: &AppContext) -> anyhow::Result<()> {
        let events = ctx.events();

        // Auto-purge cache on content updates
        events.on("post.published", |event| async move {
            // Purge cache for the published post URL
            Ok(())
        }).await?;

        events.on("post.updated", |event| async move {
            // Purge cache for the updated post URL
            Ok(())
        }).await?;

        events.on("media.uploaded", |event| async move {
            // Optionally sync to R2 if enabled
            Ok(())
        }).await?;

        events.on("settings.updated", |event| async move {
            // Reinitialize client if Cloudflare settings changed
            Ok(())
        }).await?;

        Ok(())
    }

    /// Start background tasks (cron jobs)
    async fn start_background_tasks(&self, ctx: &AppContext) -> anyhow::Result<()> {
        // Background tasks are managed by the cron system defined in plugin.toml
        Ok(())
    }

    /// Stop background tasks
    async fn stop_background_tasks(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl LifecycleHook for RustCloudflarePlugin {
    async fn on_activate(&self, context: &ActivationContext) -> Result<(), HookError> {
        info!("RustCloudflare lifecycle: on_activate");
        Ok(())
    }

    async fn on_deactivate(&self, context: &DeactivationContext) -> Result<(), HookError> {
        info!("RustCloudflare lifecycle: on_deactivate");
        Ok(())
    }

    async fn on_upgrade(&self, context: &UpgradeContext) -> Result<(), HookError> {
        info!(
            "RustCloudflare lifecycle: upgrading from {} to {}",
            context.from_version, context.to_version
        );
        Ok(())
    }

    async fn on_uninstall(&self, context: &UninstallContext) -> Result<(), HookError> {
        info!("RustCloudflare lifecycle: on_uninstall");
        // Clean up plugin data if requested
        if context.delete_data {
            // Remove plugin tables and data
        }
        Ok(())
    }

    async fn on_init(&self, context: &InitContext) -> Result<(), HookError> {
        info!("RustCloudflare lifecycle: on_init");
        Ok(())
    }

    async fn on_load(&self, context: &LoadContext) -> Result<(), HookError> {
        info!("RustCloudflare lifecycle: on_load");
        Ok(())
    }

    async fn on_shutdown(&self, context: &ShutdownContext) -> Result<(), HookError> {
        info!("RustCloudflare lifecycle: on_shutdown");
        Ok(())
    }
}

/// Plugin entry point - called by RustPress plugin loader
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(RustCloudflarePlugin::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let plugin = RustCloudflarePlugin::new();
        assert_eq!(plugin.info().id, PLUGIN_ID);
        assert_eq!(plugin.info().name, "RustCloudflare");
    }

    #[test]
    fn test_plugin_state() {
        let plugin = RustCloudflarePlugin::new();
        assert!(matches!(plugin.state(), PluginState::Inactive));
    }
}
