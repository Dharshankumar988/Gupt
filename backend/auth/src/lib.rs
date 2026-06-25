//! # Gupt Auth
//!
//! Authentication service for the Gupt backend. Provides JWT access token
//! generation and validation (HS256), refresh token generation and hashing,
//! and challenge-response helpers.

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Errors that can occur during authentication operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AuthError {
    /// The provided credentials are invalid.
    #[error("invalid credentials")]
    InvalidCredentials,

    /// The access token has expired.
    #[error("token expired")]
    TokenExpired,

    /// The access token is malformed or otherwise invalid.
    #[error("token invalid: {0}")]
    TokenInvalid(String),

    /// The requested user does not exist.
    #[error("user not found")]
    UserNotFound,

    /// A user with the given username already exists.
    #[error("user already exists")]
    UserAlreadyExists,

    /// An unexpected internal error.
    #[error("internal error: {0}")]
    InternalError(String),
}

/// Claims embedded in a JWT access token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject — the user's UUID as a string.
    pub sub: String,
    /// The device identifier that requested this token.
    pub device_id: String,
    /// Expiration time (seconds since UNIX epoch).
    pub exp: usize,
    /// Issued-at time (seconds since UNIX epoch).
    pub iat: usize,
    /// Unique token identifier (JWT ID).
    pub jti: String,
}

/// Configuration for the authentication service.
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// The HMAC secret used to sign and verify JWTs.
    pub jwt_secret: String,
    /// Lifetime of access tokens in seconds (default: 3600 = 1 hour).
    pub access_token_ttl_seconds: i64,
    /// Lifetime of refresh tokens in seconds (default: 2592000 = 30 days).
    pub refresh_token_ttl_seconds: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: String::new(),
            access_token_ttl_seconds: 3600,
            refresh_token_ttl_seconds: 2_592_000,
        }
    }
}

/// The authentication service responsible for token lifecycle management.
#[derive(Debug, Clone)]
pub struct AuthService {
    /// The active configuration.
    pub config: AuthConfig,
}

impl AuthService {
    /// Creates a new [`AuthService`] with the given configuration.
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    /// Generates a signed JWT access token for the given user and device.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InternalError`] if token encoding fails.
    pub fn generate_access_token(
        &self,
        user_id: &Uuid,
        device_id: &str,
    ) -> Result<String, AuthError> {
        let now = Utc::now().timestamp() as usize;
        let exp = now + self.config.access_token_ttl_seconds as usize;

        let claims = JwtClaims {
            sub: user_id.to_string(),
            device_id: device_id.to_string(),
            exp,
            iat: now,
            jti: Uuid::new_v4().to_string(),
        };

        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let key = EncodingKey::from_secret(self.config.jwt_secret.as_bytes());

        encode(&header, &claims, &key)
            .map_err(|e| AuthError::InternalError(format!("JWT encoding failed: {e}")))
    }

    /// Generates a cryptographically random 64-byte refresh token,
    /// returned as a 128-character hex string.
    pub fn generate_refresh_token(&self) -> String {
        let mut buf = [0u8; 64];
        rand::thread_rng().fill(&mut buf);
        hex::encode(buf)
    }

    /// Validates a JWT access token and returns the embedded claims.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::TokenExpired`] if the token is expired, or
    /// [`AuthError::TokenInvalid`] for any other validation failure.
    pub fn validate_access_token(&self, token: &str) -> Result<JwtClaims, AuthError> {
        let key = DecodingKey::from_secret(self.config.jwt_secret.as_bytes());
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.validate_exp = true;

        match decode::<JwtClaims>(token, &key, &validation) {
            Ok(data) => Ok(data.claims),
            Err(e) => {
                if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                    Err(AuthError::TokenExpired)
                } else {
                    Err(AuthError::TokenInvalid(e.to_string()))
                }
            }
        }
    }

    /// Returns the SHA-256 hash of the given refresh token as a hex string.
    pub fn hash_refresh_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Generates a random challenge string for the challenge-response
    /// authentication flow. Returns a 64-character hex string.
    pub fn generate_challenge(&self, _user_id: &Uuid) -> String {
        let mut buf = [0u8; 32];
        rand::thread_rng().fill(&mut buf);
        hex::encode(buf)
    }
}
