#![deny(warnings)]

use std::path::{Path, PathBuf};

const INCLUDE_DIRS: [&str; 7] = [
    "src/fake-zlib",
    "src/deps/ots/include",
    "src/deps/lz4/lib",
    "src/deps/brotli/c/dec",
    "src/deps/brotli/c/include",
    "src/deps/woff2/include",
    "src/deps/woff2/src",
];

fn build_lz4() {
    cc::Build::new()
        .file("src/deps/lz4/lib/lz4.c")
        .includes(INCLUDE_DIRS)
        .compile("lz4");
}

fn build_ots() {
    let ots_sources = glob::glob("src/deps/ots/src/*.cc")
        .expect("Invalid glob pattern")
        .collect::<Result<Vec<PathBuf>, _>>()
        .expect("vendored ots sources not found");

    cc::Build::new()
        .cpp(true)
        .files(ots_sources)
        .include("src/deps/ots/include")
        .include("src/deps/lz4/lib")
        .includes(INCLUDE_DIRS)
        .std("c++11")
        .compile("ots");
}

fn build_brotli() {
    let brotli_sources = glob::glob("src/deps/brotli/c/**/*.c")
        .expect("Invalid glob pattern")
        .collect::<Result<Vec<PathBuf>, _>>()
        .expect("vendored brotli sources not found");

    cc::Build::new()
        .files(brotli_sources)
        .includes(INCLUDE_DIRS)
        .compile("brotli");
}

fn build_woff2() {
    let woff2_dir = Path::new("src/deps/woff2/src");
    let file_names = ["table_tags.cc",
    "variable_length.cc",
    "woff2_common.cc",
    "woff2_dec.cc",
    "woff2_out.cc"];
    let woff2_sources = file_names.iter().map(|name| woff2_dir.join(name));

    cc::Build::new()
        .files(woff2_sources)
        .includes(INCLUDE_DIRS)
        .std("c++11")
        .warnings(false)
        .compile("woff2");
}

fn build_ots_glue() {
    cc::Build::new()
        .file("src/ots_glue.cc")
        .includes(INCLUDE_DIRS)
        .std("c++11")
        .compile("ots_glue");
}


fn main() {
    build_lz4();
    build_ots();
    build_brotli();
    build_woff2();
    build_ots_glue();
}
