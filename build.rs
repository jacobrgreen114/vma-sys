// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use bindgen;
use std::env::var;
use std::path::PathBuf;

fn main() {
    let vulkan_sdk_path = PathBuf::from(var("VULKAN_SDK").expect("VULKAN_SDK not set"));
    let vulkan_include_dir: PathBuf = vulkan_sdk_path.join("Include");

    let vma_header_rel_path = PathBuf::from("vma").join("vk_mem_alloc.h");
    let vma_header_path = vulkan_include_dir.join(vma_header_rel_path);

    (!vma_header_path.exists()).then(|| {
        panic!("VMA header not found at {:?}", vma_header_path);
    });

    cc::Build::new()
        .cpp(false)
        .include(&vulkan_include_dir)
        .file("src/vma.cpp")
        .compile("vma");

    let bindings = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-I", vulkan_include_dir.to_str().unwrap()])
        .header(vma_header_path.to_str().unwrap())
        .allowlist_recursively(false)
        .allowlist_file(".*vk_mem_alloc.*")
        // .allowlist_type("Vma.*")
        // .allowlist_type("PFN_vma.*")
        // .allowlist_function("vma.*")
        // .allowlist_var("VMA.*")
        .generate_cstr(true)
        .derive_default(true)
        .derive_debug(true)
        .impl_debug(true)
        .derive_partialeq(true)
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
