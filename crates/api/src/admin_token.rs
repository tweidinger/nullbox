
use std::sync::RwLock;

use subtle::ConstantTimeEq;

pub struct AdminTokenProvider {
    current: RwLock<String>,
}

impl AdminTokenProvider {
    pub fn new(token: String) -> Self {
        Self {
            current: RwLock::new(token),
        }
    }

    pub fn verify(&self, provided: &str) -> bool {
        let current = match self.current.read() {
            Ok(token) => token,
            Err(_) => return false,
        };

        bool::from(
            provided
                .as_bytes()
                .ct_eq(current.as_bytes())
        )
    }

    pub fn rotate(&self, new_token: String) -> Result<(), &'static str> {
        if new_token.len() < 32 {
            return Err("Admin token must be at least 32 bytes");
        }

        let mut current = self.current.write()
            .map_err(|_| "Token lock unavailable")?;

        *current = new_token;

        Ok(())
    }
}
