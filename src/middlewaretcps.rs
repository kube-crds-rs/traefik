// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f middlewaretcps.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// MiddlewareTCPSpec defines the desired state of a MiddlewareTCP.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "traefik.io",
    version = "v1alpha1",
    kind = "MiddlewareTCP",
    plural = "middlewaretcps"
)]
#[kube(namespaced)]
pub struct MiddlewareTCPSpec {
    /// InFlightConn defines the InFlightConn middleware configuration.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "inFlightConn"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub in_flight_conn: Option<MiddlewareTCPInFlightConn>,
    /// IPAllowList defines the IPAllowList middleware configuration.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ipAllowList"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ip_allow_list: Option<MiddlewareTCPIpAllowList>,
}

/// InFlightConn defines the InFlightConn middleware configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct MiddlewareTCPInFlightConn {
    /// Amount defines the maximum amount of allowed simultaneous connections. The middleware closes the connection if there are already amount connections opened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub amount: Option<i64>,
}

/// IPAllowList defines the IPAllowList middleware configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct MiddlewareTCPIpAllowList {
    /// SourceRange defines the allowed IPs (or ranges of allowed IPs by using CIDR notation).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sourceRange"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub source_range: Option<Vec<String>>,
}