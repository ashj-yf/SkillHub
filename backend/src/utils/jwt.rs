use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user id
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_token(user_id: &str, role: &str, secret: &str, expiration_hours: i64) -> Result<String> {
    let now = chrono::Utc::now();
    let exp = now
        .checked_add_signed(chrono::Duration::hours(expiration_hours))
        .ok_or_else(|| anyhow::anyhow!("Invalid expiration time"))?
        .timestamp() as usize;

    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp,
        iat,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    let mut validation = Validation::default();
    validation.set_required_spec_claims(&["exp", "iat", "sub", "role"]);

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    // 检查 token 是否过期
    let now = chrono::Utc::now().timestamp() as usize;
    if data.claims.exp < now {
        return Err(anyhow::anyhow!("Token 已过期"));
    }

    Ok(data.claims)
}