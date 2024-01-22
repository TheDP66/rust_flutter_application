use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hash(password: impl Into<String>) -> Result<String, String> {
    let password = password.into();

    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(format!(
            "Password must not be more than {} characters",
            MAX_PASSWORD_LENGTH
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| "Error while hashing password")?
        .to_string();

    Ok(hashed_password)
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(format!(
            "Password must not be more than {} characters",
            MAX_PASSWORD_LENGTH
        ));
    }

    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|_| "Invalid password hash format".to_string())?;

    let password_matches = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    Ok(password_matches)
}
