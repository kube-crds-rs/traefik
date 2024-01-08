// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f ingressrouteudps.yml --schema=derived --docs -b
// kopium version: 0.16.2

use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// IngressRouteUDPSpec defines the desired state of a IngressRouteUDP.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "traefik.io",
    version = "v1alpha1",
    kind = "IngressRouteUDP",
    plural = "ingressrouteudps"
)]
#[kube(namespaced)]
pub struct IngressRouteUDPSpec {
    /// EntryPoints defines the list of entry point names to bind to. Entry points have to be configured in the static configuration. More info: https://doc.traefik.io/traefik/v3.0/routing/entrypoints/ Default: all.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "entryPoints"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub entry_points: Option<Vec<String>>,
    /// Routes defines the list of routes.
    #[cfg_attr(feature = "builder", builder(default))]
    pub routes: Vec<IngressRouteUDPRoutes>,
}

/// RouteUDP holds the UDP route configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct IngressRouteUDPRoutes {
    /// Services defines the list of UDP services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub services: Option<Vec<IngressRouteUDPRoutesServices>>,
}

/// ServiceUDP defines an upstream UDP service to proxy traffic to.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct IngressRouteUDPRoutesServices {
    /// Name defines the name of the referenced Kubernetes Service.
    pub name: String,
    /// Namespace defines the namespace of the referenced Kubernetes Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub namespace: Option<String>,
    /// NativeLB controls, when creating the load-balancer, whether the LB's children are directly the pods IPs or if the only child is the Kubernetes Service clusterIP. The Kubernetes Service itself does load-balance to the pods. By default, NativeLB is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nativeLB")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub native_lb: Option<bool>,
    /// Port defines the port of a Kubernetes Service. This can be a reference to a named port.
    pub port: IntOrString,
    /// Weight defines the weight used when balancing requests between multiple Kubernetes Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub weight: Option<i64>,
}