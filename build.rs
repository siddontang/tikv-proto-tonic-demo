use std::path::PathBuf;

fn get_proto_files(proto_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let proto_files = std::fs::read_dir(proto_path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "proto"))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    Ok(proto_files)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let include_path = "./include";

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let descriptor_set_path = out_dir.join("kvproto.bin");

    let proto_path = "./proto";
    let proto_files = get_proto_files(proto_path)?;

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_set_path)
        .include_file("mod.rs")
        .build_server(true)
        .generate_default_stubs(true)
        .compile(&proto_files, &[include_path, proto_path])?;

    Ok(())
}
