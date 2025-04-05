use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user ID (as a string/UUID)
    pub exp: usize,   // expiration timestamp
}
