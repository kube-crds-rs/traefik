#!/usr/bin/env python3

import yaml
import requests
from time import sleep
import tempfile
import subprocess


def pascal_to_snake(s):
    return "".join(["_" + c.lower() if c.isupper() else c for c in s]).lstrip("_")


rust_lib = """//! Kubernetes CRDs for Traefik 3.0
//!
//! This library provides automatically generated types for the [Traefik 3.0 CRD definitions]. It is
//! intended to be used with the [Kube-rs] library.
//!
//! [Traefik 3.0 CRD definitions]: https://raw.githubusercontent.com/traefik/traefik/v3.0/integration/fixtures/k8s/01-traefik-crd.yml
//! [Kube-rs]: https://kube.rs/

"""

# Download https://raw.githubusercontent.com/traefik/traefik/v3.0/integration/fixtures/k8s/01-traefik-crd.yml and save it as traefik-crd.yml
crds = yaml.safe_load_all(
    requests.get(
        "https://raw.githubusercontent.com/traefik/traefik/v3.0/integration/fixtures/k8s/01-traefik-crd.yml"
    ).text
)
for crd in crds:
    file_name = crd["metadata"]["name"].removesuffix(".traefik.io")
    rust_code = ""
    if file_name == "middlewares":
        del crd["spec"]["versions"][0]["schema"]["openAPIV3Schema"]["properties"][
            "spec"
        ]["properties"]["plugin"]
    # Save the CRD as a tmp yaml file
    with tempfile.NamedTemporaryFile(mode="w") as f:
        yaml.dump(crd, f)
        tmp_file = f.name
        rust_code = subprocess.run(
            ["kopium", "-f", tmp_file, "--schema=derived", "--docs", "-b"],
            capture_output=True,
        )
        if rust_code.returncode != 0:
            print(rust_code.stderr.decode("utf-8"))
            exit(1)
        rust_code = rust_code.stdout.decode("utf-8")

    rust_code = rust_code.replace(
        f"// kopium command: kopium -f {tmp_file} --schema=derived --docs -b",
        f"// kopium command: kopium -f {file_name}.yml --schema=derived --docs -b",
    )
    rust_code = "\n".join(
        [
            line.replace("#[builder(", '#[cfg_attr(feature = "builder", builder(')
            .strip()
            .removesuffix("]")
            + ")]"
            if "#[builder(" in line
            else line
            for line in rust_code.split("\n")
        ]
    )
    # We're not setting PartialEq, Hash, Default with kopium because then rustfmt would insert a line break, which would make this script more complicated
    rust_code = rust_code.replace(
        ", TypedBuilder, JsonSchema)]\npub struct",
        ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct",
    )
    rust_code = rust_code.replace(
        ", TypedBuilder, JsonSchema)]\npub enum",
        ", PartialEq, TypedBuilder, JsonSchema)]\npub enum",
    )
    rust_code = rust_code.replace(
        "#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, TypedBuilder, JsonSchema)]\npub enum IngressRouteRoutesKind {\n",
        "#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, TypedBuilder, JsonSchema)]\npub enum IngressRouteRoutesKind {\n#[default]\n",
    )
    rust_code = "\n".join(
        [
            line.replace(
                ", TypedBuilder, JsonSchema)]",
                ')]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\n#[cfg_attr(not(feature = "schemars"), kube(schema="disabled"))]',
            )
            if line.startswith("#[derive(") and "CustomResource" in line
            else line.replace(
                ", TypedBuilder, JsonSchema)]",
                ')]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]',
            )
            if line.startswith("#[derive(")
            else line
            for line in rust_code.split("\n")
        ]
    )
    rust_code = (
        rust_code.replace(
            "use typed_builder::TypedBuilder;",
            '#[cfg(feature = "builder")]\nuse typed_builder::TypedBuilder;',
        )
        .replace(
            "use schemars::JsonSchema;",
            '#[cfg(feature = "schemars")]\nuse schemars::JsonSchema;',
        )
        .replace("use kube::CustomResource;", "use kube_derive::CustomResource;")
        .replace(
            '#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\npub enum',
            '#[cfg_attr(feature = "schemars", derive(JsonSchema))]\npub enum',
        )
    )
    if file_name == "middlewares":
        rust_code = rust_code.replace(
            "pub struct MiddlewareSpec {",
            'pub struct MiddlewareSpec {\n/// Plugin defines the middleware plugin configuration. More info: https://doc.traefik.io/traefik/plugins/\n#[serde(default, skip_serializing_if = "Option::is_none")]\n#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]\nplugin: Option<BTreeMap<String, serde_json::Value>>,',
        )
    rust_file = f"./src/{file_name}.rs"
    with open(rust_file, "w") as f:
        f.write(rust_code)
    # Format the code
    subprocess.run(["rustfmt", rust_file])
    rust_lib += f"pub mod {file_name};\npub use {file_name}::*;\n"

with open("./src/lib.rs", "w") as f:
    f.write(rust_lib)
