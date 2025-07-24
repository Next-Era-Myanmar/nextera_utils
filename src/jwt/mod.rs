use crate::time::Time;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{Duration, NaiveDateTime};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// ### Default claim struct for authentication.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,    // subject (user ID)
    pub org: i32,    // organization ID
    pub exp: usize,  // expiration timestamp
    pub suid: String, // session uuid (UUID or unique session)
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
/// match validate_jwt(access_token, secret, audience){
///     Ok(result)=>{
///         assert_eq!(result.claims.aud.as_str(), audience);
///     },
///     Err(e)=>{
///             println!("{}" ,e)
///         }
///     };
/// ```
pub fn validate_jwt(
    token: &str,
    secret: &str,
    audience: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.set_audience(&[audience]);
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

/// ### Get Jwt Claims with validation.
///
/// ### Example
///
/// ```
/// use nextera_utils::jwt::{get_jwt_claims_from_token};
/// let access_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImV4cCI6MTczMjIwMDQ3NywiaXNzIjoiTmV4dCBFcmEgQXV0aGVudGljYWl0b24gU2VydmljZSIsImF1ZCI6Ik5FWFQgRVJBIFVTRVIifQ.dSFOwqIq_FtTTU1GuB7KVROgQP6sjtfWRLtozG-JrR4";
/// let success_result:i32 = 3;
/// match get_jwt_claims_from_token(access_token){
///     Ok(result)=>{
///         assert_eq!(result.sub, success_result);
///     },
///     Err(e)=>{
///             println!("{}" ,e)
///         }
///     };
/// ```
pub fn get_jwt_claims_from_token(token: &str) -> Result<Claims, String> {
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

    Ok(claims)
}

/// ### Generate JWT Token with Secret.
///
/// ### Example
///
/// ```
/// use nextera_utils::jwt::{generate_jwt};
/// let user_id = 1;
/// let org_id = 1;
/// let secret = "YourOrgSecret";
/// let success_result:i32 = 3;
/// match generate_jwt(user_id, org_id, secret, 3600, "Next Era Authentication Service", "NEXTERA USER"){
///     Ok(result)=>{
///             println!("{}" ,result.0.as_str());
///             assert_eq!(result.0.len() > 0, true);
///     },
///     Err(e)=>{
///             println!("{}" ,e)
///         }
///     };
/// ```
pub fn generate_jwt<'a>(
    user_id: i32,
    org_id: i32,
    secret: &str,
    expires_in_sec: i64,
    session_uuid: &str,
    audience: &str,
) -> Result<(String, NaiveDateTime), &'a str> {
    let expire_datetime = Time::get_utc()
        .checked_add_signed(Duration::seconds(expires_in_sec))
        .expect("valid timestamp");
    let expire_timestamp = expire_datetime.and_utc().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        org: org_id.to_owned(),
        exp: expire_timestamp,
        suid: session_uuid.to_owned(),
        aud: audience.to_owned(),
    };

    Ok((
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
            .expect("Error creating token"),
        expire_datetime,
    ))
}

fn normalize_base64(input: &str) -> String {
    let mut normalized = input.to_string();
    while normalized.len() % 4 != 0 {
        normalized.push('='); // Add padding
    }
    normalized
}
