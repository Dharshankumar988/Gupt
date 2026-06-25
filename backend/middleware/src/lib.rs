//! # Gupt Middleware
//!
//! Axum middleware for JWT-based authentication. Extracts the `Bearer` token
//! from the `Authorization` header, validates it via [`gupt_auth::AuthService`],
//! and injects an [`AuthenticatedUser`] into request extensions.

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use gupt_auth::AuthService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Errors that can occur in the authentication middleware.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MiddlewareError {
    /// The Authorization header is missing or malformed.
    #[error("missing or invalid authorization header")]
    MissingAuth,

    /// The JWT token is invalid or expired.
    #[error("authentication failed: {0}")]
    AuthFailed(String),
}

/// Represents an authenticated user extracted from a valid JWT.
///
/// This is inserted into Axum request extensions by [`auth_middleware`] and
/// can be retrieved in handlers via `Extension<AuthenticatedUser>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    /// The authenticated user's UUID.
    pub user_id: Uuid,
    /// The device identifier from the JWT claims.
    pub device_id: String,
}

/// Axum middleware that validates JWT Bearer tokens.
///
/// Extracts the token from the `Authorization: Bearer <token>` header,
/// validates it against the [`AuthService`], and injects an
/// [`AuthenticatedUser`] into the request extensions on success.
///
/// Returns `401 Unauthorized` if the token is missing, malformed, or invalid.
pub fn auth_middleware(
    State(auth): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send + 'static>,
> {
    Box::pin(async move {
        let auth_header = request
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims = auth
            .validate_access_token(token)
            .map_err(|e| {
                tracing::warn!(error = %e, "JWT validation failed");
                StatusCode::UNAUTHORIZED
            })?;

        let user_id = claims
            .sub
            .parse::<Uuid>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        request.extensions_mut().insert(AuthenticatedUser {
            user_id,
            device_id: claims.device_id,
        });

        Ok(next.run(request).await)
    })
}

/// Creates an Axum middleware layer that performs JWT authentication.
///
/// Use this when constructing the router to protect routes:
///
/// ```ignore
/// let protected = Router::new()
///     .route("/protected", get(handler))
///     .route_layer(create_auth_layer(auth_service));
/// ```
pub fn create_auth_layer(
    auth_service: Arc<AuthService>,
) -> axum::middleware::FromFnLayer<
    fn(
        State<Arc<AuthService>>,
        Request,
        Next,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>,
    >,
    Arc<AuthService>,
    (),
> {
    // We cannot directly return a named fn-pointer closure type because
    // auth_middleware is async. Instead, use `from_fn_with_state` which
    // infers the type from the function signature.
    axum::middleware::from_fn_with_state(auth_service, auth_middleware)
}
