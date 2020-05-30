use crate::models::user::SlimUser;
use crate::utils::env::ENV;
use actix_web::{dev, http::header, Error, FromRequest, HttpRequest, HttpResponse};
use chrono::{Duration, Local};
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::Value;
use std::convert::From;

pub type LoggedUser = SlimUser;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user uuid
    id: String,
}

impl Claims {
    fn with_id(id: &str) -> Self {
        Claims {
            iss: ENV.domain.clone(),
            sub: "auth".into(),
            id: id.to_string(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            id: Some(claims.id),
            token: None,
        }
    }
}

pub fn create_token(id: &str) -> Option<String> {
    let claims = Claims::with_id(id);
    let mut header = Header::default();
    header.alg = Algorithm::HS512;
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(ENV.jwt_private_key.as_ref()),
    )
    .ok()
}

pub fn decode_token(token: &str) -> Option<SlimUser> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(ENV.jwt_private_key.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map(|data| data.claims.into())
    .ok()
}

pub fn token_from_value(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Object(data) => data
            .get("authToken")
            .or(data.get("authKey"))
            .or(data.get("authorization"))
            .or(data.get("Authorization"))
            .map(|value| match value {
                Value::String(token) => token.clone(),
                _ => "".to_string(),
            }),
        _ => None,
    }
}

impl FromRequest for SlimUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get(header::AUTHORIZATION);
        let token = match auth {
            Some(header_value) => match header_value.to_str() {
                Ok(value) => Some(value.replace("Bearer", "").trim().to_string()),
                Err(_) => None,
            },
            None => None,
        };
        match token {
            None => return ok(SlimUser::default()),
            Some(token) => {
                let user = decode_token(&token).map_or(SlimUser::default(), move |mut user| {
                    user.token = Some(token);
                    user
                });
                ok(user)
            }
        }
    }
}
