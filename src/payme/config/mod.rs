use std::env;

pub fn get_recaptcha_secret() -> String {
    env::var("RECAPTCHA_SECRET").unwrap()
}

pub fn get_crypto_secret() -> String {
    env::var("CRYPTO_SECRET").unwrap()
}

pub fn get_host() -> String {
    env::var("HOST").unwrap()
}
