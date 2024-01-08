// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f tlsoptions.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// TLSOptionSpec defines the desired state of a TLSOption.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "traefik.io",
    version = "v1alpha1",
    kind = "TLSOption",
    plural = "tlsoptions"
)]
#[kube(namespaced)]
pub struct TLSOptionSpec {
    /// ALPNProtocols defines the list of supported application level protocols for the TLS handshake, in order of preference. More info: https://doc.traefik.io/traefik/v3.0/https/tls/#alpn-protocols
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "alpnProtocols"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub alpn_protocols: Option<Vec<String>>,
    /// CipherSuites defines the list of supported cipher suites for TLS versions up to TLS 1.2. More info: https://doc.traefik.io/traefik/v3.0/https/tls/#cipher-suites
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cipherSuites"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub cipher_suites: Option<Vec<String>>,
    /// ClientAuth defines the server's policy for TLS Client Authentication.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientAuth"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub client_auth: Option<TLSOptionClientAuth>,
    /// CurvePreferences defines the preferred elliptic curves in a specific order. More info: https://doc.traefik.io/traefik/v3.0/https/tls/#curve-preferences
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "curvePreferences"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub curve_preferences: Option<Vec<String>>,
    /// MaxVersion defines the maximum TLS version that Traefik will accept. Possible values: VersionTLS10, VersionTLS11, VersionTLS12, VersionTLS13. Default: None.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "maxVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub max_version: Option<String>,
    /// MinVersion defines the minimum TLS version that Traefik will accept. Possible values: VersionTLS10, VersionTLS11, VersionTLS12, VersionTLS13. Default: VersionTLS10.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "minVersion"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub min_version: Option<String>,
    /// SniStrict defines whether Traefik allows connections from clients connections that do not specify a server_name extension.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sniStrict")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sni_strict: Option<bool>,
}

/// ClientAuth defines the server's policy for TLS Client Authentication.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TLSOptionClientAuth {
    /// ClientAuthType defines the client authentication type to apply.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientAuthType"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub client_auth_type: Option<TLSOptionClientAuthClientAuthType>,
    /// SecretNames defines the names of the referenced Kubernetes Secret storing certificate details.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "secretNames"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub secret_names: Option<Vec<String>>,
}

/// ClientAuth defines the server's policy for TLS Client Authentication.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum TLSOptionClientAuthClientAuthType {
    NoClientCert,
    RequestClientCert,
    RequireAnyClientCert,
    VerifyClientCertIfGiven,
    RequireAndVerifyClientCert,
}
