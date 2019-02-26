use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let proto_root = "src/protos";
    let output = "src/rpc";
    let protos = ["raft.proto"];
    println!("cargo:rerun-if-changed={}", proto_root);
    if !Path::new(output).exists() {
        fs::create_dir_all(output).unwrap()
    }

    protoc_grpcio::compile_grpc_protos(&protos, &[proto_root], &output)
        .expect("Failed to compile gRPC definitions!");
    let dir = Path::new(output);
    let mut mod_rs = File::create(Path::new(output).join("mod.rs")).unwrap();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file = path.file_name().unwrap().to_str().unwrap();
            if file == "mod.rs" {
                continue;
            }
            let (module_name, _) = file.split_at(file.len() - 3); // ".rs".len() == 3
            writeln!(mod_rs, "pub mod {};", module_name).unwrap();
        }
    }
}
