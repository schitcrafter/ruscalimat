use std::sync::OnceLock;

use jsonwebtoken::{jwk::JwkSet, DecodingKey, TokenData, Validation};
use poem::{
    error::{BadRequest, InternalServerError, Unauthorized},
    http::StatusCode,
    Endpoint, Request, Result,
};
use serde::Deserialize;
use tracing::debug;

pub async fn setup(auth_server_url: &str) -> color_eyre::Result<()> {
    let openid_configuration: serde_json::Value = reqwest::get(format!(
        "{auth_server_url}/.well-known/openid-configuration"
    ))
    .await?
    .json()
    .await?;

    debug!(
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

static JWK_SET: OnceLock<JwkSet> = OnceLock::new();

pub async fn get_jwk_set(jwks_url: &str) -> color_eyre::Result<JwkSet> {
    reqwest::get(jwks_url)
        .await?
        .json()
        .await
        .map_err(From::from)
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserClaims {
    #[serde(default)]
    pub groups: Vec<String>,
}

/// If an auth header was sent, this validates it and
/// adds user info as data to the request. If
pub async fn auth_middleware<E: Endpoint>(next: E, mut req: Request) -> Result<E::Output> {
    if let Some(auth_token) = req.header("Authorization") {
        let auth_token = auth_token
            .strip_prefix("Bearer ")
            .ok_or(poem::Error::from_string(
                "Invalid auth header",
                StatusCode::BAD_REQUEST,
            ))?;

        let unverified_header = jsonwebtoken::decode_header(auth_token).map_err(BadRequest)?;

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

        // FIXME: Store `DecodingKey`, not `JwkSet`
        let decoding_key = DecodingKey::from_jwk(jwk).map_err(InternalServerError)?;
        let algorithm = jwk
            .common
            .algorithm
            .ok_or(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        let validation = Validation::new(algorithm);

        let verified_header: TokenData<UserClaims> =
            jsonwebtoken::decode(auth_token, &decoding_key, &validation).map_err(Unauthorized)?;

        debug!("Adding claims to data: {:?}", verified_header.claims);
        req.extensions_mut().insert(verified_header.claims);
    }
    next.call(req).await
}
