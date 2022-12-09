use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &String) -> String {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password_hash: &String, password: &String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
