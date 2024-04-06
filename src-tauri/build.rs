fn main() {
    use std::env;

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let mut cxx = cxx_build::bridge("src/libtorrent.rs");
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let libtorrent_dir = "C:\\install-torrent-lib\\include";
    let boost_dir = "C:\\boost_1_82_0";

    cxx.include(libtorrent_dir)
    .include(boost_dir)
	.include(manifest_dir)
    .file("src/lt.cc")
	.include(out_dir)
	.flag_if_supported("-std=c++14")
	.compile("libtorrent-ffi");

    println!("cargo:rustc-flags=-llibtorrent-rasterbar");
    println!("cargo:rustc-link-search=native=C:\\boost_1_82_0\\stage\\lib");
    println!("cargo:rustc-flags=-llibcrypto_static");
    println!("cargo:rustc-flags=-llibssl_static");

    println!("cargo:rerun-if-changed=src/libtorrent.rs");
    println!("cargo:rerun-if-changed=src/lt.h");
    println!("cargo:rerun-if-changed=src/lt.cc");

    tauri_build::build()
}
