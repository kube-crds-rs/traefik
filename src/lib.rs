//! Kubernetes CRDs for Traefik v3.1.0
//!
//! This library provides automatically generated types for the [Traefik v3.1.0 CRD definitions]. It is
//! intended to be used with the [Kube-rs] library.
//!
//! [Traefik v3.1.0 CRD definitions]: https://raw.githubusercontent.com/traefik/traefik/v3.1.0/integration/fixtures/k8s/01-traefik-crd.yml
//! [Kube-rs]: https://kube.rs/

pub mod ingressroutes;
pub use ingressroutes::*;
pub mod ingressroutetcps;
pub use ingressroutetcps::*;
pub mod ingressrouteudps;
pub use ingressrouteudps::*;
pub mod middlewares;
pub use middlewares::*;
pub mod middlewaretcps;
pub use middlewaretcps::*;
pub mod serverstransports;
pub use serverstransports::*;
pub mod serverstransporttcps;
pub use serverstransporttcps::*;
pub mod tlsoptions;
pub use tlsoptions::*;
pub mod tlsstores;
pub use tlsstores::*;
pub mod traefikservices;
pub use traefikservices::*;
