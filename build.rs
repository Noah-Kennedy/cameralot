fn main() {
    // invoke cmake and ninja
    let mut cfg = cmake::Config::new("cameralot");

    let dst = if cfg!(target_os = "windows") {
        &mut cfg
    } else {
        cfg.generator("Ninja")
    }.build().join("lib");

    // only see this if run with -vv
    println!("Path is {}\n", dst.to_str().unwrap());

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=cameralot-capture");
    println!("cargo:rustc-link-lib=dylib=opencv_core");
    println!("cargo:rustc-link-lib=dylib=opencv_imgproc");
    println!("cargo:rustc-link-lib=dylib=opencv_imgcodecs");
    println!("cargo:rustc-link-lib=dylib=opencv_videoio");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}