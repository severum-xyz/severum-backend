use serde::{Deserialize, Serialize};

/// Represents the claims used in JWT tokens.
///
/// This struct includes the subject (user identifier) and token expiration time.
#[derive(Clone, Deserialize, Serialize)]
pub struct Claims {
    /// The subject of the token, typically the user ID.
    pub sub: String,

    /// The expiration time of the token, represented as a Unix timestamp.
    pub exp: usize,
}
