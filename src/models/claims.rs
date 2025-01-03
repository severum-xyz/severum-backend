use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
