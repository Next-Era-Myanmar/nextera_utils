use base64::Engine;
use base64::engine::general_purpose;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// ### Default claim struct for authentication.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,    // subject (user ID)
    pub exp: usize,  // expiration timestamp
    pub iss: String, // issuer (UUID or unique session)
    pub aud: String, // audience (Service Name)
}

/// ### Check jwt token for authentication.
///
/// ### Example
///
/// ```
/// use nextera_utils::jwt::{validate_jwt, Claims};
/// let access_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTczMjIwMDQ3NywiaXNzIjoiTmV4dCBFcmEgQXV0aGVudGljYWl0b24gU2VydmljZSIsImF1ZCI6Ik5FWFQgRVJBIFVTRVIifQ.dSFOwqIq_FtTTU1GuB7KVROgQP6sjtfWRLtozG-JrR4";
/// let secret = "ACCESS_SECRET_2024!@#super_secure_random_string_1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// let audience = "NEXT ERA USER";
/// match validate_jwt(access_token,secret,audience){
///     Ok(result)=>{
///         assert_eq!(result.claims.aud.as_str(),audience);
///     },
///     Err(e)=>{
///             println!("{}" ,e)
///         }
///     };
/// ```
pub fn validate_jwt(
    token: &str,
    secret: &str,
    expected_audience: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.set_audience(&[expected_audience]);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
}

/// ### Get user id from token.
///
/// ### Example
///
/// ```
/// use nextera_utils::jwt::{get_user_id_from_token};
/// let access_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTczMjIwMDQ3NywiaXNzIjoiTmV4dCBFcmEgQXV0aGVudGljYWl0b24gU2VydmljZSIsImF1ZCI6Ik5FWFQgRVJBIFVTRVIifQ.dSFOwqIq_FtTTU1GuB7KVROgQP6sjtfWRLtozG-JrR4";
/// let success_result:i32 = 3;
/// match get_user_id_from_token(access_token){
///     Ok(result)=>{
///         assert_eq!(result,success_result);
///     },
///     Err(e)=>{
///             println!("{}" ,e)
///         }
///     };
/// ```
pub fn get_user_id_from_token(token: &str) -> Result<i32, String> {
    // Split the token into header, payload, and signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }

    // Normalize and decode the payload (Base64 URL decoding)
    let normalized_payload = normalize_base64(parts[1]);
    let payload = general_purpose::URL_SAFE
        .decode(normalized_payload)
        .map_err(|e| format!("Base64 decoding failed: {}", e))?;

    // Convert payload to a string
    let payload_str =
        String::from_utf8(payload).map_err(|e| format!("Invalid UTF-8 in payload: {}", e))?;

    // Deserialize JSON into Claims
    let claims: Claims = serde_json::from_str(&payload_str)
        .map_err(|e| format!("Failed to deserialize claims: {}", e))?;

    Ok(claims.sub)
}

fn normalize_base64(input: &str) -> String {
    let mut normalized = input.to_string();
    while normalized.len() % 4 != 0 {
        normalized.push('='); // Add padding
    }
    normalized
}
