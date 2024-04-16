// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use bindgen;
use std::env::var;
use std::path::PathBuf;

macro_rules! cargo_warning {
    ($($arg:tt)*) => {
        println!("cargo:warning={}", format!($($arg)*));
    };
}

macro_rules! cargo_panic {
    ($($arg:tt)*) => {
        cargo_warning!($($arg)*);
        panic!($($arg)*);
    };
}

// const RUSTIFIED_ENUMS: &[&str] = &["VkObjectType"];
// const BITFIELD_ENUMS: &[&str] = &[];

fn main() {
    let vulkan_sdk_path = PathBuf::from(var("VULKAN_SDK").expect("VULKAN_SDK not set"));
    let vulkan_include_dir: PathBuf = vulkan_sdk_path.join("Include");

    let vma_header_rel_path = PathBuf::from("vma").join("vk_mem_alloc.h");
    let vma_header_path = vulkan_include_dir.join(vma_header_rel_path);

    (!vma_header_path.exists()).then(|| {
        cargo_panic!("VMA header not found at {:?}", vma_header_path);
    });

    cc::Build::new()
        .cpp(false)
        .include(&vulkan_include_dir)
        .file("src/vma.cpp")
        .compile("vma");

    let mut builder = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-I", vulkan_include_dir.to_str().unwrap()])
        // .clang_args(&["-L", var("OUT_DIR").unwrap().as_str()])
        // .clang_args(&["-D", platform_define])
        .header(vma_header_path.to_str().unwrap())
        // .blocklist_file(".*vulkan.*")
        .allowlist_recursively(false)
        .allowlist_file(".*vk_mem_alloc.*")
        // .allowlist_type("Vma.*")
        // .allowlist_type("PFN_vma.*")
        // .allowlist_function("vma.*")
        // .allowlist_var("VMA.*")
        .prepend_enum_name(false);

    // for rustified in RUSTIFIED_ENUMS {
    //     builder = builder.rustified_non_exhaustive_enum(rustified)
    // }

    // for bitfield in BITFIELD_ENUMS {
    //     builder = builder.bitfield_enum(bitfield)
    // }

    let bindings = builder
        .generate()
        .inspect_err(|e| {
            cargo_panic!("{}", e);
        })
        .unwrap();

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .inspect_err(|e| {
            cargo_panic!("{}", e);
        })
        .unwrap();
}
