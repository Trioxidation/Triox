use argon2::{self, Config};
use rand::prelude::*;

pub fn from_password(password: &str) -> Result<String, &str> {
    // generate random salt value for the hash function
    // the salt value will be stored inside the hash string
    let salt = rand::thread_rng().gen::<[u8; 20]>();

    // use default config
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|_| "Failed to create hash")
}

pub fn compare_passwords<'a>(password: &[u8], hash: &'a str) -> Result<bool, &'a str> {
    argon2::verify_encoded(hash, password).map_err(|_| "Failed to verify password")
}
