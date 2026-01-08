//! Service layer for RustCloudflare
//!
//! Business logic and orchestration for Cloudflare operations

pub mod cache;
pub mod dns;
pub mod security;
pub mod workers;
pub mod r2;
pub mod analytics;
pub mod settings;
pub mod oauth;

use crate::client::CloudflareClient;
use crate::error::CloudflareResult;
use sqlx::PgPool;
use std::sync::Arc;

pub use settings::{SettingsService, CloudflareCredentials, PluginSettings};
pub use oauth::{OAuthService, OAuthConfig, TokenResources};

/// Main services container
pub struct CloudflareServices {
    pub cache: cache::CacheService,
    pub dns: dns::DnsService,
    pub security: security::SecurityService,
    pub workers: workers::WorkersService,
    pub r2: r2::R2Service,
    pub analytics: analytics::AnalyticsService,
    pub settings: settings::SettingsService,
    pub oauth: oauth::OAuthService,
}

impl CloudflareServices {
    /// Create a new services container
    pub fn new(client: Arc<CloudflareClient>, db: PgPool) -> Self {
        Self {
            cache: cache::CacheService::new(Arc::clone(&client), db.clone()),
            dns: dns::DnsService::new(Arc::clone(&client), db.clone()),
            security: security::SecurityService::new(Arc::clone(&client), db.clone()),
            workers: workers::WorkersService::new(Arc::clone(&client), db.clone()),
            r2: r2::R2Service::new(Arc::clone(&client), db.clone()),
            analytics: analytics::AnalyticsService::new(Arc::clone(&client), db.clone()),
            settings: settings::SettingsService::new(db.clone()),
            oauth: oauth::OAuthService::new(),
        }
    }

    /// Create services without a client (for initial setup)
    pub fn new_unconfigured(db: PgPool) -> Self {
        // Create a dummy client for services that need it
        // These services will fail gracefully if called without proper config
        Self {
            cache: cache::CacheService::new_unconfigured(db.clone()),
            dns: dns::DnsService::new_unconfigured(db.clone()),
            security: security::SecurityService::new_unconfigured(db.clone()),
            workers: workers::WorkersService::new_unconfigured(db.clone()),
            r2: r2::R2Service::new_unconfigured(db.clone()),
            analytics: analytics::AnalyticsService::new_unconfigured(db.clone()),
            settings: settings::SettingsService::new(db.clone()),
            oauth: oauth::OAuthService::new(),
        }
    }
}
