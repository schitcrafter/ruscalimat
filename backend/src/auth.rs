use std::sync::OnceLock;

use jsonwebtoken::{jwk::JwkSet, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use poem::{
    error::{BadRequest, InternalServerError, Unauthorized},
    http::StatusCode,
    Request, Result,
};
use poem_openapi::{auth::Bearer, SecurityScheme};
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

use crate::config::SETTINGS;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", bearer_format = "jwt", checker = "check_bearer_opt")]
pub struct JwtBearerAuth(pub UserClaims);

async fn check_bearer_opt(_req: &Request, bearer: Bearer) -> Option<UserClaims> {
    check_bearer(bearer).await.ok()
}

pub async fn check_bearer(bearer: Bearer) -> Result<UserClaims> {
    debug!("Checking bearer {}", bearer.token);

    let unverified_header =
        jsonwebtoken::decode_header(bearer.token.as_str()).map_err(BadRequest)?;

    let kid = unverified_header.kid.ok_or(poem::Error::from_string(
        "JWT needs kid (key id) claim!",
        StatusCode::BAD_REQUEST,
    ))?;

    let jwk = JWK_SET
        .get()
        .unwrap()
        .find(&kid)
        .ok_or(poem::Error::from_string(
            format!("Could not find key id {}", kid),
            StatusCode::BAD_REQUEST,
        ))?;

    let decoding_key = DecodingKey::from_jwk(jwk).map_err(InternalServerError)?;
    let algorithm = jwk
        .common
        .algorithm
        .ok_or(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
    let validation = Validation::new(algorithm);

    debug!("Trying to verify JWT");
    let verified_header = jsonwebtoken::decode(bearer.token.as_str(), &decoding_key, &validation)
        .map_err(Unauthorized)?;

    debug!("Auth successful with claims {:?}", verified_header.claims);

    Ok(verified_header.claims)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    #[serde(rename = "sub")]
    pub user_id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub groups: Vec<String>,
}

static ADMIN_GROUP: Lazy<String> = Lazy::new(|| SETTINGS.get_string("auth.admin_group").unwrap());

impl UserClaims {
    pub fn is_admin(&self) -> bool {
        self.groups
            .iter()
            .any(|group| group == ADMIN_GROUP.as_str())
    }
}

static JWK_SET: OnceLock<JwkSet> = OnceLock::new();

pub async fn setup(auth_server_url: &str) -> color_eyre::Result<()> {
    let openid_configuration: serde_json::Value = reqwest::get(format!(
        "{auth_server_url}/.well-known/openid-configuration"
    ))
    .await?
    .json()
    .await?;

    trace!(
        "Got response from openid_configuration endpoint: {}",
        openid_configuration
    );

    let jwks_url = openid_configuration.get("jwks_uri").unwrap();
    let jwks_url = jwks_url.as_str().unwrap();
    debug!("jwks_url: {jwks_url}");

    let jwkset = get_jwk_set(jwks_url).await?;
    JWK_SET.set(jwkset).unwrap();
    debug!("Got jwk set");
    Ok(())
}

async fn get_jwk_set(jwks_url: &str) -> color_eyre::Result<JwkSet> {
    reqwest::get(jwks_url)
        .await?
        .json()
        .await
        .map_err(From::from)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinUserClaims {
    #[serde(rename = "sub")]
    pub user_id: String,
}

static PIN_JWT_KEY: Lazy<EncodingKey> = Lazy::new(get_jwt_encoding_key);

fn get_jwt_encoding_key() -> EncodingKey {
    let key_path = SETTINGS.get_string("auth.pinlogin.keypath").unwrap();
    let key = std::fs::read(key_path.clone())
        .expect(format!("You need to put a private pem key at {key_path}").as_str());
    trace!(
        "Read private key pem file: {}",
        String::from_utf8(key.clone()).unwrap()
    );
    EncodingKey::from_ec_pem(&key).expect("Not a valid private ec pem key")
}

pub fn create_pin_jwt(user_id: &str) -> jsonwebtoken::errors::Result<String> {
    let header = Header::new(jsonwebtoken::Algorithm::ES256);

    let claims = PinUserClaims {
        user_id: user_id.to_owned(),
    };

    let jwt = jsonwebtoken::encode(&header, &claims, &*PIN_JWT_KEY)?;

    Ok(format!("Pin {jwt}"))
}
