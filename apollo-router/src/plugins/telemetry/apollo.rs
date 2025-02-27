//! Configuration for apollo telemetry.
// This entire file is license key functionality
use crate::graphql::plugin::utils::serde::deserialize_header_name;
use http::header::HeaderName;
use schemars::JsonSchema;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub endpoint: Option<Url>,

    #[schemars(with = "Option<String>", default = "apollo_key_env_str")]
    #[serde(default = "apollo_key")]
    pub apollo_key: Option<String>,

    #[schemars(with = "Option<String>", default = "apollo_graph_ref_env_str")]
    #[serde(default = "apollo_graph_reference")]
    pub apollo_graph_ref: Option<String>,

    #[schemars(with = "Option<String>", default = "client_name_header_default_str")]
    #[serde(
        deserialize_with = "deserialize_header_name",
        default = "client_name_header_default"
    )]
    pub client_name_header: HeaderName,

    #[schemars(with = "Option<String>", default = "client_version_header_default_str")]
    #[serde(
        deserialize_with = "deserialize_header_name",
        default = "client_version_header_default"
    )]
    pub client_version_header: HeaderName,
}

fn apollo_key_env_str() -> Option<String> {
    Some("${APOLLO_KEY}".to_string())
}

fn apollo_graph_ref_env_str() -> Option<String> {
    Some("${APOLLO_GRAPH_REF}".to_string())
}

fn apollo_key() -> Option<String> {
    std::env::var("APOLLO_KEY").ok()
}

fn apollo_graph_reference() -> Option<String> {
    std::env::var("APOLLO_GRAPH_REF").ok()
}

fn client_name_header_default_str() -> &'static str {
    "apollographql-client-name"
}

fn client_name_header_default() -> HeaderName {
    HeaderName::from_static(client_name_header_default_str())
}

fn client_version_header_default_str() -> &'static str {
    "apollographql-client-version"
}

fn client_version_header_default() -> HeaderName {
    HeaderName::from_static(client_version_header_default_str())
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: None,
            apollo_key: None,
            apollo_graph_ref: None,
            client_name_header: client_name_header_default(),
            client_version_header: client_version_header_default(),
        }
    }
}
