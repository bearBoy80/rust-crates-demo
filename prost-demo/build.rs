use std::io::Result;

use prost_build::Config;
fn main() -> Result<()> {
    let mut config = Config::new();
    //指定编译输出路径
    config.out_dir("src/pb")
    .message_attribute(".", "#[derive(Eq)]")
    //
    .field_attribute("User.age", "#[serde(rename = \"in\")]")
    .type_attribute("User", "#[derive(serde::Serialize, serde::Deserialize)]")
    //指定编译文件
    .compile_protos(&["protoc/user.proto"], &["protoc"])?;
    Ok(())
}