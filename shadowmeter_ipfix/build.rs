extern crate pkg_config;

fn main() {

    // compile options
    let src = ["src/yaf_file_process.c"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())      
        .include("/usr/include/glib-2.0")
        .include("/usr/lib/x86_64-linux-gnu/glib-2.0/include")        
        .flag("-Wno-unused-parameter")
        .opt_level(2);
    build.compile("yaf_file_process");

    // link options
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=fixbuf");
    println!("cargo:rustc-link-lib=glib-2.0");

}
