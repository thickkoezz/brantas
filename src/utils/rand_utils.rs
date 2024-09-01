use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use rand::Rng;
use std::iter;
use rust_i18n::t;

///  Generate a string of a specified length
#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
  iter::repeat(())
    .map(|_| rand::thread_rng().sample(rand::distributions::Alphanumeric))
    .map(char::from)
    .take(limit)
    .collect()
}

pub async fn verify_password(password: String, password_hash: String) -> anyhow::Result<()> {
  tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
    let hash = PasswordHash::new(&password_hash)
      .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    let result = hash.verify_password(&[&Argon2::default()], password);
    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(anyhow::anyhow!(t!("invalid_x", x = t!("password")))),
    }
  })
    .await
    .context("panic in verifying password hash")?
}

pub async fn hash_password(password: String, mut salt: SaltString) -> anyhow::Result<String> {
  tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
    if salt.to_string().is_empty() {
      salt = SaltString::generate(rand::thread_rng());
    }
    Ok(PasswordHash::generate(Argon2::default(), password, &salt)
      .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
      .to_string())
  })
    .await
    .context("panic in generating password hash")?
}
