// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f serverstransporttcps.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// ServersTransportTCPSpec defines the desired state of a ServersTransportTCP.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "traefik.io",
    version = "v1alpha1",
    kind = "ServersTransportTCP",
    plural = "serverstransporttcps"
)]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct ServersTransportTCPSpec {
    /// DialKeepAlive is the interval between keep-alive probes for an active network connection. If zero, keep-alive probes are sent with a default value (currently 15 seconds), if supported by the protocol and operating system. Network protocols or operating systems that do not support keep-alives ignore this field. If negative, keep-alive probes are disabled.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dialKeepAlive"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub dial_keep_alive: Option<IntOrString>,
    /// DialTimeout is the amount of time to wait until a connection to a backend server can be established.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "dialTimeout"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub dial_timeout: Option<IntOrString>,
    /// TerminationDelay defines the delay to wait before fully terminating the connection, after one connected peer has closed its writing capability.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "terminationDelay"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub termination_delay: Option<IntOrString>,
    /// TLS defines the TLS configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub tls: Option<ServersTransportTCPTls>,
}

/// TLS defines the TLS configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ServersTransportTCPTls {
    /// CertificatesSecrets defines a list of secret storing client certificates for mTLS.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "certificatesSecrets"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub certificates_secrets: Option<Vec<String>>,
    /// InsecureSkipVerify disables TLS certificate verification.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "insecureSkipVerify"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub insecure_skip_verify: Option<bool>,
    /// MaxIdleConnsPerHost controls the maximum idle (keep-alive) to keep per-host.
    /// PeerCertURI defines the peer cert URI used to match against SAN URI during the peer certificate verification.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "peerCertURI"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peer_cert_uri: Option<String>,
    /// RootCAsSecrets defines a list of CA secret used to validate self-signed certificates.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rootCAsSecrets"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub root_c_as_secrets: Option<Vec<String>>,
    /// ServerName defines the server name used to contact the server.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "serverName"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub server_name: Option<String>,
    /// Spiffe defines the SPIFFE configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub spiffe: Option<ServersTransportTCPTlsSpiffe>,
}

/// Spiffe defines the SPIFFE configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ServersTransportTCPTlsSpiffe {
    /// IDs defines the allowed SPIFFE IDs (takes precedence over the SPIFFE TrustDomain).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ids: Option<Vec<String>>,
    /// TrustDomain defines the allowed SPIFFE trust domain.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustDomain"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub trust_domain: Option<String>,
}
