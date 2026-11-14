//! Nullbox API: the HTTP layer, services and storage around `nullbox-domain`.
//!
//! Extension seams, each in one place:
//! - auth: [`http::extract::Actor`]
//! - authorization: [`policy::can`]
//! - audit/webhooks/metrics: [`events::EventSink`]
//! - middleware (tracing, CORS, rate limits...): [`http::middleware`]
//! - storage: [`repo::TicketRepository`]
//! - settings: [`config::Config`]


pub mod config;
pub mod db;
pub mod dto;
pub mod error;
pub mod events;
pub mod http;
pub mod policy;
pub mod repo;
pub mod service;
pub mod token;
pub mod admin_token;

use std::sync::Arc;

use admin_token::AdminTokenProvider;

pub use config::Config;
pub use error::ApiError;

/// Shared state for all handlers.
#[derive(Clone)]
pub struct AppState {
    pub tickets: Arc<service::TicketService>,
    pub config: Arc<Config>,
    pub admin_token: Arc<AdminTokenProvider>,
}

impl AppState {
    pub fn new(pool: sqlx::SqlitePool, config: Config) -> Self {
        let repo = Arc::new(repo::SqliteTicketRepository::new(pool));
        let events = Arc::new(events::TracingEventSink);

        let admin_token = Arc::new(
            AdminTokenProvider::new(config.admin_token.clone())
        );

        Self {
            tickets: Arc::new(service::TicketService::new(repo, events)),
            config: Arc::new(config),
            admin_token,
        }
    }
}
