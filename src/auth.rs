use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::Argon2;

use argon2::password_hash::{PasswordHash, PasswordVerifier};

use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

pub fn hash_password(password: & str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("hashing failed: {e}"))?;
    Ok(hash.to_string())

    
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,       //"subject" - what the token is about
    pub exp: usize,      //expiry, as a Unix timestamp
}

pub fn make_token(user_id: Uuid, secret: &str) -> anyhow::Result<String> {
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;
    let claims = Claims {sub: user_id, exp };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)

}