use crate::config::Config;
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AuthUrl, Client, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl,
    StandardRevocableToken, TokenUrl,
};
pub type OAuthBasicClient = Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub fn build_google_client(config: &Config) -> OAuthBasicClient {
    let auth_uri = AuthUrl::new(config.google_auth_uri.clone()).expect("Invalid Google auth URI");
    let token_uri = TokenUrl::new(config.google_token_uri.clone()).expect("Invalid Google token URI");
    let redirect_uri = RedirectUrl::new(config.google_redirect_uri.clone()).expect("Invalid Google redirect URI");

    BasicClient::new(ClientId::new(config.google_client_id.clone()))
        .set_client_secret(ClientSecret::new(config.google_client_secret.clone()))
        .set_auth_uri(auth_uri)
        .set_token_uri(token_uri)
        .set_redirect_uri(redirect_uri)
}