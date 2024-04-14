// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=../presence_core/src/lib.rs");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(&crate_dir)
        .unwrap()
        .write_to_file("presence.h");

    // This is the directory where the `c` library is located.
    let lib_dir_path = PathBuf::from(crate_dir.as_str()).join("hello").join("hello.h")
        .canonicalize()
        .expect(&*format!("cannot canonicalize path: {}", crate_dir.as_str()));

    // This is the path to the `c` headers file.
    let headers_path_str = lib_dir_path.to_str().expect("Path not valid");

    println!("cargo:rustc-link-search={}/build/hello", crate_dir);
    println!("cargo:rustc-link-lib=hello");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("presence_provider.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");


}