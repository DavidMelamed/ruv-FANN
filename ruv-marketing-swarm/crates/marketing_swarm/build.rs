fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/marketing.proto"], &["proto"])
        .unwrap();

    println!("cargo:rerun-if-changed=proto/marketing.proto");
}
