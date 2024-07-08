fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .file_descriptor_set_path("helloworld_descriptor.bin")
        .compile(&["protoc/hello.proto"], &["protoc"])
        .unwrap();
}
