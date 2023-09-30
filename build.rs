use prost_wrapper::{GenOpt, WrapperGen};
use std::io::Write;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

fn get_proto_files(proto_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let proto_files = std::fs::read_dir(proto_path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "proto"))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    Ok(proto_files)
}

fn list_rs_files(out_dir: &str) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(out_dir)
        .expect("Couldn't read directory")
        .filter_map(|e| {
            let path = e.expect("Couldn't list file").path();
            if path.extension() == Some(std::ffi::OsStr::new("rs")) {
                Some(path)
            } else {
                None
            }
        })
}

fn generate_mod_file(out_dir: &str) {
    let mut f = File::create(format!("{}/mod.rs", out_dir)).unwrap();

    let modules = list_rs_files(out_dir).filter_map(|path| {
        let name = path.file_stem().unwrap().to_str().unwrap();
        if name.starts_with("wrapper_") || name == "mod" {
            return None;
        }
        Some((name.replace('-', "_"), name.to_owned()))
    });

    let mut exports = String::new();
    for (module, file_name) in modules {
        let mut level = 0;
        for part in module.split('.') {
            writeln!(f, "pub mod {} {{", part).unwrap();
            level += 1;
        }
        writeln!(f, "include!(\"{}.rs\");", file_name,).unwrap();
        if Path::new(&format!("{}/wrapper_{}.rs", out_dir, file_name)).exists() {
            writeln!(f, "include!(\"wrapper_{}.rs\");", file_name,).unwrap();
        }
        writeln!(f, "{}", "}\n".repeat(level)).unwrap();
    }

    // if !exports.is_empty() {
    //     writeln!(
    //         f,
    //         "pub mod {} {{ {} }}",
    //         self.package_name.as_ref().unwrap(),
    //         exports
    //     )
    //     .unwrap();
    // }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let include_path = "./include";

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let descriptor_set_path = out_dir.join("kvproto.bin");

    let proto_path = "./proto";
    let proto_files = get_proto_files(proto_path)?;

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_set_path)
        // .include_file("mod.rs")
        .build_server(true)
        .generate_default_stubs(true)
        .compile(&proto_files, &[include_path, proto_path])?;

    println!("out_dir={}", out_dir.to_str().unwrap());

    list_rs_files(out_dir.to_str().unwrap()).for_each(|path| {
        println!("path={}", path.to_str().unwrap());
        WrapperGen::new(path, GenOpt::all()).write()
    });

    generate_mod_file(out_dir.to_str().unwrap());

    Ok(())
}
