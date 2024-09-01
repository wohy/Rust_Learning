// build.rs 可以在编译 cargo 项目时，做额外的编译处理。
fn main() {
    // 这里我们使用 prost_build 把 abi.proto 编译到 src/pb 目录下。
    prost_build::Config::new()
    .out_dir("src/pb")
    .compile_protos(&["abi.proto"], &["."])
    .unwrap();
}