use argon2::{self, Config};
use rand::Rng;

/// Hash a senha antes de salvar no banco de dados
pub fn hash_password(password: &str) -> String {
    let salt: [u8; 16] = rand::thread_rng().gen();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

/// Verifique a senha durante o login
pub fn verify_password(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}
