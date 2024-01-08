//! Kubernetes CRDs for Traefik 3.0
//!
//! This library provides automatically generated types for the [Traefik 3.0 CRD definitions]. It is
//! intended to be used with the [Kube-rs] library.
//!
//! [Traefik 3.0 CRD definitions]: https://raw.githubusercontent.com/traefik/traefik/v3.0/integration/fixtures/k8s/01-traefik-crd.yml
//! [Kube-rs]: https://kube.rs/

mod ingressroutes;
pub use ingressroutes::*;
mod ingressroutetcps;
pub use ingressroutetcps::*;
mod ingressrouteudps;
pub use ingressrouteudps::*;
mod middlewares;
pub use middlewares::*;
mod middlewaretcps;
pub use middlewaretcps::*;
mod serverstransports;
pub use serverstransports::*;
mod serverstransporttcps;
pub use serverstransporttcps::*;
mod tlsoptions;
pub use tlsoptions::*;
mod tlsstores;
pub use tlsstores::*;
mod traefikservices;
pub use traefikservices::*;
