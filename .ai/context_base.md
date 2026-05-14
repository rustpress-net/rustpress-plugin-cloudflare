# rustpress-plugin-cloudflare — AI Context

> **Purpose**: Orient an AI agent to this repo without reading the whole tree. Pair with the RustPress organisation context in `rustpress-core-base/.ai/context/CONTEXT_BASE.md`.

## Project

`rustpress-plugin-cloudflare` (crate name `rustcloudflare`) is a **comprehensive Cloudflare platform integration** for RustPress: CDN cache purging, DNS CRUD, WAF rules, SSL/TLS management, Workers deployment, R2 object storage, D1 database, Stream video, OAuth SSO via Cloudflare. The plugin ships both a Rust backend and a TypeScript/React admin UI (`admin-ui/`) built with Vite.

Audit verdict: **🔴 NOT READY** → ship as `0.4.0-beta`. The OAuth-token-in-URL bug has been fixed (recent commit `78df937 fix(security): stop leaking OAuth access token in redirect URL`). Path-deps in Cargo.toml and missing tests still block promotion to 1.0.

## Tech stack

- **Language**: Rust 2021 (backend) + TypeScript/React (admin UI)
- **Crate**: `rustcloudflare` v1.0.1, MIT
- **Web**: `axum 0.7` + `tower` + `tower-http` (cors, trace, compression-gzip), `hyper 1.1`
- **HTTP client**: `reqwest 0.11` with `rustls-tls`, `multipart`, `stream`
- **Async**: `tokio 1.35` (full), `async-trait`, `futures`
- **DB**: `sqlx 0.7` (postgres + mysql + sqlite + uuid + chrono + json)
- **Validation**: `validator 0.16` (derive)
- **Frontend**: Vite + TypeScript + React in `admin-ui/`
- **Plans / planning docs**: `DEVELOPMENT_PLAN.md`, `BACKEND_API_PLAN.md` at repo root

## Directory layout

```
rustpress-plugin-cloudflare/
├── Cargo.toml              # crate = rustcloudflare, v1.0.1, MIT
├── plugin.toml             # plugin manifest
├── README.md               # 21KB, accurate, feature-rich
├── LICENSE                 # MIT
├── DEVELOPMENT_PLAN.md     # internal roadmap
├── BACKEND_API_PLAN.md     # API design doc
├── package.json            # tooling for the admin UI build orchestration
├── src/
│   ├── lib.rs              # Plugin trait impl, registration
│   ├── client.rs           # Cloudflare HTTP client wrapper
│   ├── config.rs
│   ├── error.rs
│   ├── api/                # incl. oauth.rs (CSRF UUID state, fixed token handling)
│   ├── hooks/
│   ├── middleware/
│   ├── models/
│   ├── services/
│   └── workers/            # Cloudflare Workers deployment helpers
├── migrations/
│   └── 1.0.0_initial_schema.sql
├── admin-ui/               # React + Vite SPA
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
├── assets/
└── scripts/
```

## Public API / what this repo exposes

- Implements RustPress `Plugin` trait
- REST API for: cache purging, DNS records, WAF, SSL/TLS, page rules, Workers (deploy/list/delete), R2 (CRUD + presigned URLs), D1 (query/migrate), Stream (upload/list)
- OAuth flow under `src/api/oauth.rs` — UUID-state CSRF protection; **post-fix**: token returned via POST/session, no longer in URL query (`oauth.rs:164` previously held the leak)
- Admin UI mounted at the plugin's admin page slot, calling the backend API
- Service trait abstractions in `src/services/` for `oauth.save_credentials()` style calls

## How to build / test

```bash
# Backend
cargo build --release
cargo test                 # WARN: no /tests/ directory; only what's inline

# Frontend
cd admin-ui && npm install && npm run build

# To build standalone (outside the core workspace) — currently BLOCKED by path deps
# in Cargo.toml referencing ../../crates/rustpress-{core,plugins,database,api,events}.
# Standalone builds require switching those to git deps (cf. visual-queue's approach).
```

CI: `rustpress-net/rustpress-core-devops/actions/ci-rust@main` + `ci-node@main` for admin-ui.

## Cross-repo dependencies

- **Depends on**: `rustpress-core-base` via **path** deps in `Cargo.toml` (`rustpress-core`, `rustpress-plugins`, `rustpress-database`, `rustpress-api`, `rustpress-events`). This implies tight coupling to a specific repo layout — the plugin currently won't build unless the core workspace is co-located. Compare with `rustpress-plugin-visual-queue` which uses git deps.
- **Depended on by**: optional — host apps that integrate with Cloudflare. Not a transitive requirement of other RustPress plugins.
- **External**: Cloudflare API v4 (`api.cloudflare.com/client/v4/`), Cloudflare OAuth, R2 (S3-compatible), D1, Stream.

## Conventions

- **License**: MIT (LICENSE file at root; align to `MIT OR Apache-2.0` to match org)
- **Commits**: Conventional Commits — recent commit messages show security-fix style (`fix(security): ...`)
- **Token storage**: via `services.oauth.save_credentials()` — encryption-at-rest assumed; verify before GA
- **OAuth state**: UUID generated and stored server-side for CSRF protection (`oauth.rs:87`)
- **HTTP**: prefer `reqwest` with `rustls-tls`; no OpenSSL deps

## Status

- Release readiness: **🔴 NOT READY** → ship as `0.4.0-beta` (see `AUDIT-plugins.md` section 4 and master `AUDIT.md`)
- The Phase 0.5 OAuth-token-in-URL security bug has been fixed in the most recent commit.
- Still blocked from 1.0 by: path-deps, missing tests, encryption story not documented.
- Cargo.toml at v1.0.1 — should be **demoted** to `0.4.0-beta` for the v1.0 release tier.

## Known issues / TODOs

From `AUDIT-plugins.md` (section 4) and master audit:

- **P0 (fixed)**: ~~OAuth access token in URL query string (`src/api/oauth.rs:164`)~~ — addressed in commit `78df937`. Verify the fix by walking the redirect flow end-to-end and confirming the token never appears in `Location:` headers, browser address bar, or referer headers.
- **P0**: No `/tests/` directory. Dev-deps include `mockall`, `wiremock`, `proptest` but no test files. Highest-priority targets: OAuth token exchange + refresh + revocation, Cloudflare API retry/backoff, R2 credential storage, sqlx-migration application. Target: 15+ tests with security focus.
- **P0**: Path deps in `Cargo.toml` block standalone distribution. Switch to git deps (`git = "https://github.com/rustpress-net/rustpress-core-base.git"`) like `visual-queue` does.
- **P1**: Document the token storage encryption strategy in README — currently assumed.
- **P1**: Align license to `MIT OR Apache-2.0`.
- **P1**: `repository` URL in `Cargo.toml` is `github.com/rustpress/rustcloudflare` — should be `rustpress-net/rustpress-plugin-cloudflare`.
- **P1**: README badge says `RustPress >=1.0.0` but plugin will ship at 0.4.0-beta — reconcile messaging.

## When working in this repo

- This plugin handles **OAuth tokens, R2 credentials, and Cloudflare API keys**. Every change to credential-handling code requires explicit test coverage in the same PR. No exceptions.
- The OAuth flow is the security boundary — touching `src/api/oauth.rs` requires extra review.
- Path-dep refactor: when switching to git deps, ensure version pinning (`tag = "v1.0.0"` once core ships) so a breaking change in core-base doesn't silently break this plugin's CI.
- The admin UI in `admin-ui/` is its own npm package with its own `package.json`; treat it as a sub-project. Don't commit `admin-ui/node_modules` or `admin-ui/dist`.
- Cloudflare API is rate-limited (1200 req / 5min by default). Implement retry-after-aware backoff in the client.
- Never log full credentials or tokens, even in debug builds. Use `tracing` with redaction.
