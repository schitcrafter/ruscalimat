use std::sync::OnceLock;

use jsonwebtoken::{jwk::JwkSet, DecodingKey, Validation};
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
#[oai(ty = "bearer", bearer_format = "jwt", checker = "check_bearer")]
pub struct JwtBearerAuth(pub UserClaims);

pub async fn check_bearer(_req: &Request, bearer: Bearer) -> Result<UserClaims> {
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
