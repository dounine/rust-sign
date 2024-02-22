extern crate cc;

fn main() {
    let mut builder: cc::Build = cc::Build::new();
    builder
        .file("src/c_code/library.c")
        .shared_flag(false)
        .compile("hello_for_ccode");

    //搜索动态库的路径，这里是相对路径，相对于build.rs所在的目录
    //可同时支持静态跟动态库
    //请先去lib文件夹下面执行make命令，生成动态库
    //```bash
    // cd lib
    // mkdir build && cd build
    // cmake ..
    // make
    // ```
    // cd lib_shared && xmake
    // println!("cargo:rustc-link-lib=dylib=hello_shared");
    // println!("cargo:rustc-link-search=native=./lib_shared/build/macosx/arm64/release/");

    // cd cpp_shared && xmake
    // println!("cargo:rustc-link-lib=dylib=cpp_shared");
    // println!("cargo:rustc-link-search=native=./cpp_shared/build/macosx/arm64/release/");

    // cd zsign && xmake
    println!("cargo:rustc-link-lib=dylib=zsign");
    // println!("cargo:rustc-link-search=native=/Users/lake/dounine/github/ipadump/zsign/build/macosx/arm64/release");
    println!("cargo:rustc-link-search=native=./zsign/tmp/");

    // cd lib_static && xmake
    // println!("cargo:rustc-link-lib=static=hello_static");
    // println!("cargo:rustc-link-search=native=./lib_static/build/macosx/arm64/release/");
}