// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f tlsstores.yml --schema=derived --docs -b
// kopium version: 0.16.2

use kube_derive::CustomResource;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// TLSStoreSpec defines the desired state of a TLSStore.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "traefik.io",
    version = "v1alpha1",
    kind = "TLSStore",
    plural = "tlsstores"
)]
#[kube(namespaced)]
pub struct TLSStoreSpec {
    /// Certificates is a list of secret names, each secret holding a key/certificate pair to add to the store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub certificates: Option<Vec<TLSStoreCertificates>>,
    /// DefaultCertificate defines the default certificate configuration.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "defaultCertificate"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub default_certificate: Option<TLSStoreDefaultCertificate>,
    /// DefaultGeneratedCert defines the default generated certificate configuration.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "defaultGeneratedCert"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub default_generated_cert: Option<TLSStoreDefaultGeneratedCert>,
}

/// Certificate holds a secret name for the TLSStore resource.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TLSStoreCertificates {
    /// SecretName is the name of the referenced Kubernetes Secret to specify the certificate details.
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// DefaultCertificate defines the default certificate configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TLSStoreDefaultCertificate {
    /// SecretName is the name of the referenced Kubernetes Secret to specify the certificate details.
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// DefaultGeneratedCert defines the default generated certificate configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TLSStoreDefaultGeneratedCert {
    /// Domain is the domain definition for the DefaultCertificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub domain: Option<TLSStoreDefaultGeneratedCertDomain>,
    /// Resolver is the name of the resolver that will be used to issue the DefaultCertificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub resolver: Option<String>,
}

/// Domain is the domain definition for the DefaultCertificate.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TLSStoreDefaultGeneratedCertDomain {
    /// Main defines the main domain name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub main: Option<String>,
    /// SANs defines the subject alternative domain names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub sans: Option<Vec<String>>,
}
