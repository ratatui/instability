fn main() {
    println!("cargo:rustc-check-cfg=cfg(instability_exclude_unstable_docs)");
}
