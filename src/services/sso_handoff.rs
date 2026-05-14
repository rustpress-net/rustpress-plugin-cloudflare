//! In-memory handoff store for the OAuth multi-account selection flow.
//!
//! When a user authenticates with Cloudflare and has more than one
//! account/zone, the server used to redirect to the admin UI with the
//! access token embedded in the URL query string. That exposes the token
//! to access logs, browser history, and referrer headers.
//!
//! This store keeps the token server-side, keyed by a short-lived random
//! UUID. The redirect only carries the UUID; the frontend exchanges it
//! for the (non-token) account/zone resources, then completes the flow
//! by quoting the same UUID back to the server.

use crate::services::TokenResources;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;

const HANDOFF_TTL: Duration = Duration::from_secs(600); // 10 minutes

#[derive(Clone)]
struct HandoffEntry {
    access_token: String,
    resources: TokenResources,
    expires_at: Instant,
}

#[derive(Default, Clone)]
pub struct SsoHandoffStore {
    inner: Arc<RwLock<HashMap<Uuid, HandoffEntry>>>,
}

impl SsoHandoffStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stash a token + its discovered resources and return the handoff ID.
    pub async fn insert(&self, access_token: String, resources: TokenResources) -> Uuid {
        let id = Uuid::new_v4();
        let entry = HandoffEntry {
            access_token,
            resources,
            expires_at: Instant::now() + HANDOFF_TTL,
        };
        let mut guard = self.inner.write().await;
        guard.insert(id, entry);
        // Opportunistic GC: drop anything past TTL on every insert.
        let now = Instant::now();
        guard.retain(|_, e| e.expires_at > now);
        id
    }

    /// Look up the resources for a pending handoff (does NOT expose the token).
    pub async fn peek_resources(&self, id: &Uuid) -> Option<TokenResources> {
        let guard = self.inner.read().await;
        let entry = guard.get(id)?;
        if entry.expires_at <= Instant::now() {
            return None;
        }
        Some(entry.resources.clone())
    }

    /// Consume the handoff and return the access token. The entry is removed
    /// whether the caller succeeds or fails; the token is one-shot.
    pub async fn consume(&self, id: &Uuid) -> Option<String> {
        let mut guard = self.inner.write().await;
        let entry = guard.remove(id)?;
        if entry.expires_at <= Instant::now() {
            return None;
        }
        Some(entry.access_token)
    }
}
