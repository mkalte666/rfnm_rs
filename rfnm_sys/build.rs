use bindgen::{Bindings, EnumVariation};
use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    // We need the rfnm lib and our wrapper
    // then we run bindgen on the wrapper
    // simple, right?

    let libdst = cmake::Config::new("librfnm")
        .profile("RelWithDebInfo")
        .build();

    println!("cargo:rustc-link-search=native={}/lib64", libdst.display());
    println!("cargo:rustc-link-lib=static=rfnm");
    // FIXME: build this as well?
    println!("cargo:rustc-link-lib=spdlog");
    println!("cargo:rustc-link-lib=fmt");
    println!("cargo:rustc-link-lib=usb-1.0");

    cc::Build::new()
        .cpp(true)
        .include("librfnm/include/")
        .file("librfnm_wrap/librfnm_wrap.cpp")
        .cargo_metadata(true)
        .compile("librfnm_wrap");

    // we have everything build except the bindgen

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    gen_bindings()
        .write_to_file(out_path.join("librfnm_wrap.rs"))
        .expect("Couldn't write bindings!");

    // rerun rules
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=librfnm_wrap");
    println!("cargo:rerun-if-changed=librfnm");
}

pub fn gen_bindings() -> Bindings {
    bindgen::Builder::default()
        .header("librfnm_wrap/librfnm_wrap.hpp")
        .allowlist_file("librfnm_wrap/librfnm_wrap.hpp")
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .bitfield_enum("rfnm::channel")
        .allowlist_item("rfnm::channel")
        .allowlist_item("rfnm::stream_format")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .generate()
        .expect("Failed to generate bindings")
}
