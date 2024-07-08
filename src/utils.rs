use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(plain_password: &str, hashed_password: &str) -> bool {
    match verify(plain_password, hashed_password) {
        Ok(result) => result,
        Err(_) => false,
    }
}