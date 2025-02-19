extern crate bindgen;
extern crate napi_build;

use std::fs;

fn main() {
  println!("cargo:rustc-link-search=native={}", "whisper.cpp/build/src");
  println!("cargo:rustc-link-search=native={}", "whisper.cpp/build/ggml/src");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "whisper");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "ggml-base");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "ggml-cpu");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "ggml");
  println!("cargo:rerun-if-changed={}", "whisper");
  println!("cargo:rerun-if-changed={}", "wrapper.h");

  fs::create_dir("bindings")
    .expect("Directory bindings already created");
  bindgen::Builder::default()
    .header("wrapper.h")
    .clang_arg("-Iwhisper.cpp/ggml/include")
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file("bindings/whisper.rs")
    .expect("Couldn't write bindings!");

  napi_build::setup();
}
