use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let out_dir = env::var("OUT_DIR").unwrap();

    Command::new(Path::new(&manifest_dir).join("autogen.sh"))
        .status()
        .unwrap();

    Command::new(Path::new(&manifest_dir).join("configure"))
        .status()
        .unwrap();

    cc::Build::new()
        .file("memcached.c")
        .file("hash.c")
        .file("jenkins_hash.c")
        .file("murmur3_hash.c")
        .file("slabs.c")
        .file("items.c")
        .file("assoc.c")
        .file("thread.c")
        .file("daemon.c")
        .file("stats.c")
        .file("util.c")
        .file("bipbuffer.c")
        .file("logger.c")
        .file("crawler.c")
        .file("itoa_ljust.c")
        .file("slab_automove.c")
        .file("cache.c")
        .file("authfile.c")
        .warnings(false)
        .define("HAVE_CONFIG_H", Some(""))
        .compile("memcached");

    println!("cargo:rustc-link-lib=event");
}
