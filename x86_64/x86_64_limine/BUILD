package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "x86_64_limine",
    srcs = glob(["src/**"]),
    deps = [
        "//common/kos-memory",
        "//common/kos-kernel",
        "@crate_index//:limine"
    ]
)
