use std::env;

pub fn get_recaptcha_secret() -> String {
    env::var("RECAPTCHA_SECRET").unwrap_or("secret".to_string())
}

pub fn get_crypto_secret() -> String {
    env::var("CRYPTO_SECRET").unwrap_or("secret".to_string())
}

pub fn get_host() -> String {
    env::var("HOST").unwrap_or("http://localhost:3000".to_string())
}
