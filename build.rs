extern crate napi_build;

fn main() {
  napi_build::setup();

  cc::Build::new()
    .cpp(true)
    .flag_if_supported("-std=c++17")
    .include("src_cpp")
    .include("/usr/local/include/node") // 包含 Node.js 头文件
    .include("node_modules/nan")        // 包含 NAN 头文件
    .file("src/cpp/bridge.cc")
    .compile("xprofiler_bridge");
    
  println!("cargo:rerun-if-changed=src/cpp/bridge.cc");
}
