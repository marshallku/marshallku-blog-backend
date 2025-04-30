use bcrypt::{hash, verify, BcryptError};

/// The number of rounds to use for the bcrypt hash.
const SALT_ROUNDS: u32 = 10;

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, SALT_ROUNDS)
}

/// Verify a password against a hashed password.
///
/// # Arguments
///
/// * `password` - The password to verify.
/// * `hashed_password` - The hashed password to verify against.
pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(password, hashed_password)
}
