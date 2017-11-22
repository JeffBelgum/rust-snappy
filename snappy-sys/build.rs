extern crate pkg_config;
extern crate cmake;

use std::env;
use std::fs;
use std::path::Path;

use cmake::Config;

fn main() {
    let want_static = env::var("SNAPPY_SYS_STATIC").unwrap_or(String::new()) == "1";
    let from_source = env::var("SNAPPY_SYS_STATIC_FROM_SOURCE").unwrap_or(String::new()) == "1";
    if !from_source && configure_snappy(want_static) {
        return;
    }
    build_snappy();
}

fn configure_snappy(want_static: bool) -> bool {
    // ~ try pkg_config first
    if pkg_config::probe_library("snappy").is_ok() {
        return true;
    }
    // ~ then try search in statically predefined directories
    let libsnappy_file = if want_static { "libsnappy.a" } else { "libsnappy.so" };
    if let Some(path) = first_path_with_file(libsnappy_file) {
        if want_static {
            println!("cargo:rustc-link-search={}", path);
            println!("cargo:rustc-link-lib=static=snappy");
            configure_stdcpp();
        } else {
            println!("cargo:rustc-link-search=native={}", path);
            println!("cargo:rustc-link-lib=dylib=snappy");
        }
        return true;
    }
    return false;
}

fn build_snappy() {
    let src = env::current_dir().unwrap().join("snappy");
    let dst = Config::new("snappy").build_target("snappy").build();
    let build = dst.join("build");
    println!("cargo:root={}", build.display());
    println!("cargo:rustc-link-lib=static=snappy");
    println!("cargo:rustc-link-search=native={}", build.display());
    fs::copy(src.join("snappy.h"), build.join("snappy.h")).unwrap();
    configure_stdcpp();
}

fn configure_stdcpp() {
    // From: https://github.com/alexcrichton/gcc-rs/blob/master/src/lib.rs
    let target = env::var("TARGET").unwrap();
    let cpp = if target.contains("darwin") { "c++" } else { "stdc++" };
    println!("cargo:rustc-link-lib={}", cpp);
}

fn first_path_with_file(file: &str) -> Option<String> {
    // we want to look in LD_LIBRARY_PATH and then some default folders
    if let Some(ld_path) = env::var_os("LD_LIBRARY_PATH") {
        for p in env::split_paths(&ld_path) {
            if is_file_in(file, &p) {
                return p.to_str().map(|s| String::from(s))
            }
        }
    }
    for p in vec!["/usr/lib","/usr/local/lib"] {
        if is_file_in(file, &Path::new(p)) {
            return Some(String::from(p))
        }
    }
    return None
}

fn is_file_in(file: &str, folder: &Path) -> bool {
    let full = folder.join(file);
    match fs::metadata(full) {
        Ok(ref found) if found.is_file() => true,
        _ => false
    }
}
