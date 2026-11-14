//! Admin endpoints. One so far: the export the CEO pastes into a spreadsheet every Monday.

use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use nullbox_domain::Ticket;
use subtle::ConstantTimeEq;

use super::extract::ADMIN_TOKEN_HEADER;
use crate::{ApiError, AppState};

/// Every ticket, with customer names and emails. Guarded by `X-Admin-Token`, which
/// must match `ADMIN_TOKEN` (see `config.rs`).
pub async fn export(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<Vec<Ticket>>, ApiError> {
    let token = headers
    .get(ADMIN_TOKEN_HEADER)
    .and_then(|value| value.to_str().ok());

let valid = token
    .map(|provided| {
        bool::from(
            provided
                .as_bytes()
                .ct_eq(state.config.admin_token.as_bytes())
        )
    })
    .unwrap_or(false);

if !valid {
    return Err(ApiError::Forbidden);
}
    Ok(Json(state.tickets.export().await?))
}
