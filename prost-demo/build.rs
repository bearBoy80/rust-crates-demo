use std::io::Result;

use prost_build::Config;
fn main() -> Result<()> {
    let mut config = Config::new();
    config.out_dir("src/pb")
    .compile_protos(&["protoc/user.proto"], &["protoc"])?;
    Ok(())
}