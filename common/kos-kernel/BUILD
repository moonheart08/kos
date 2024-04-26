package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "kos-kernel",
    srcs = glob(["src/**"]),
    deps = [
        "//common/kos-memory",
        "@crate_index//:spin",
        "@crate_index//:x86_64"
    ]
)
