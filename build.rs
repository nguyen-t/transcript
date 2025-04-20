extern crate bindgen;
extern crate napi_build;

use std::{env, fs};

fn main() {
  let root = env::current_dir().unwrap();

  println!("cargo:rustc-link-search=native={}", root.join("whisper.cpp/build/src").display());
  println!("cargo:rustc-link-search=native={}", root.join("whisper.cpp/build/ggml/src").display());
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "whisper");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "ggml-cpu");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "ggml-base");
  println!("cargo:rustc-link-lib=static:+whole-archive={}", "ggml");
  println!("cargo:rerun-if-changed={}", root.join("whisper").display());
  println!("cargo:rerun-if-changed={}", root.join("wrapper.h").display());

  fs::create_dir("bindings").ok();
  bindgen::Builder::default()
    .header("wrapper.h")
    .clang_arg(format!("-I{}", root.join("whisper.cpp/ggml/include").display()))
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file(root.join("bindings/whisper.rs"))
    .expect("Couldn't write bindings!");

  napi_build::setup();
}
