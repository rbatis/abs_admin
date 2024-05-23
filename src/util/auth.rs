use std::env;

use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, crypto::verify, DecodingKey, decode, Validation, Algorithm, TokenData};
use bcrypt;

use crate::models::{auth::Claims, error::{Error, Result}};

use super::JWT_DURATION_IN_SECONDS;
/// Hash a password using bcrypt
pub fn hash_password(password: String) -> Result<String> {
	bcrypt::hash(password, 12).map_err(|_| Error::InternalServerError)
}

/// Compare and verify a password with it's hash
pub fn verify_hash(password: String, hash: &str) -> Result<bool> {
	bcrypt::verify(password, hash).map_err(|_| Error::InternalServerError)
}

/// Create JWT from author's email and ID
pub fn create_jwt(email: String, id: i64) -> Result<String>{
	let jwt_secret = EncodingKey::from_secret(env::var("JWT_SECRET").expect("Env variable `JWT_SECRET` not found").as_ref());
	let mut now = Utc::now();
	let iat = (now.timestamp() as usize); // Issued at
	let expires_in = Duration::seconds(JWT_DURATION_IN_SECONDS);
	now+= expires_in;
	let exp = now.timestamp() as usize; // Expires at

	let claim = Claims {
		exp,
		iat,
		email,
		id
	};

	let token = encode(&Header::default(), &claim, &jwt_secret).map_err(|_| Error::InternalServerError);
	token
}

/// Check if JWT is valid
pub fn is_jwt_valid(token: &str) -> Result<bool>{
	let secret = env::var("JWT_SECRET").expect("Env variable `JWT_SECRET` not found");
	let key = &DecodingKey::from_secret(secret.as_bytes());
	
	let is_decoded = decode::<Claims>(token, key, &Validation::new(Algorithm::HS256)).map_err(|e| {
		match e.kind() {
			jsonwebtoken::errors::ErrorKind::ExpiredSignature => { Error::AuthFailCookieExpired },
			_ => { Error::InvalidJwt }
		}
	})?;

	Ok(true)
}

/// Decode a JWT to return the author email and author ID inside
pub fn get_info_from_jwt(token: String) -> Result<(String, i64)> {
	let secret = env::var("JWT_SECRET").expect("Env variable `JWT_SECRET` not found");
	let key = &DecodingKey::from_secret(secret.as_bytes());
	
	let is_decoded = decode::<Claims>(&token, key, &Validation::new(Algorithm::HS256)).map_err(|e| {
		match e.kind() {
			jsonwebtoken::errors::ErrorKind::ExpiredSignature => { Error::AuthFailCookieExpired },
			_ => { Error::InvalidJwt }
		}
	})?;

	Ok((is_decoded.claims.email, is_decoded.claims.id))
}