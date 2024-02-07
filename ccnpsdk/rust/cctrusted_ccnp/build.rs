use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/ccnp-server.proto")?;

    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_dir = "./src";

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("ccnp_server_descriptor.bin"))
        .compile(&["proto/ccnp-server.proto"], &["proto"])?;

    Ok(())
}
