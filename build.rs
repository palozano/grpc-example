use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // get the build artifacts directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // allow for reflection by compiling the protobuf definitions into the file descriptor set
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("calculator_descriptor.bin"))
        .compile(&["proto/calculator.proto"], &["proto"])?;

    // tonic builds everythnig with this
    tonic_build::compile_protos("proto/calculator.proto")?;

    Ok(())
}
