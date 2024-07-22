extern crate pkg_config;

fn main() {

    // compile options
    let src = ["src/lib/fixbuf_wrapper.c"];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())      
        .include("/usr/include/glib-2.0")
        .include("/usr/lib/x86_64-linux-gnu/glib-2.0/include")        
        .include("../../cert-nsa-yaf/include/yaf") 
        .include("../../cert-nsa-yaf/airframe/include/airframe")
        .flag("-Wno-unused-parameter")
        .opt_level(2);
    build.compile("yaf_processor");

    // link options
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=fixbuf");
    println!("cargo:rustc-link-lib=airframe");
    println!("cargo:rustc-link-lib=ndpi");    
    println!("cargo:rustc-link-lib=duckdb");       
    println!("cargo:rustc-link-lib=maxminddb");  
    println!("cargo:rustc-link-lib=glib-2.0");

}
