extern crate cc;

fn main() {
    let mut builder: cc::Build = cc::Build::new();
    // builder
    //     .warnings(false)
    //     .file("src/c_code/library.c")
    //     .shared_flag(false)
    //     .compile("hello_for_ccode");
    // //c++ 代码
    // builder
    //     .cpp(true)
    //     .warnings(false)
    //     .std("c++11")
    //     .flag("-c")
    //     .shared_flag(false)
    //     // .flag("-lssl")
    //     // .flag("-lcrypto")
    //     // .flag("-I/opt/homebrew/Cellar/openssl@3/3.2.0_1/include")
    //     // .flag("-L/opt/homebrew/Cellar/openssl@3/3.2.0_1/lib")
    //     .file("src/cpp_code/librarycpp.cpp")
    //     .compile("cpp_static");
    //
    // println!("cargo:rerun-if-changed=src/cpp_code/librarycpp.cpp");

    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/openssl@3/3.2.0_1/lib");
    //
    //
    // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/openssl@3/3.2.0_1/lib");


    //依赖openssl
    builder
        .cpp(true)
        .warnings(false)
        .std("c++11")
        .flag("-c")
        // .shared_flag(true)
        // .flag("-lssl")
        // .flag("-lcrypto")
        .flag("-I/opt/homebrew/Cellar/openssl@3/3.2.0_1/include")
        .flag("-L/opt/homebrew/Cellar/openssl@3/3.2.0_1/lib")
        .file("zsign/common/base64.cpp")
        .file("zsign/common/json.cpp")
        .file("zsign/common/common.cpp")
        .file("zsign/archo.cpp")
        .file("zsign/bundle.cpp")
        .file("zsign/macho.cpp")
        .file("zsign/openssl.cpp")
        .file("zsign/signing.cpp")
        .file("zsign/zsign.cpp")
        .file("zsign/main.cpp")
        .compile("zsign");

    println!("cargo:rerun-if-changed=zsign/common/common.cpp");

    // println!("cargo:rustc-link-lib=dylib=ssl");
    // println!("cargo:rerun-if-changed=zsign/common/common.cpp");
    // println!("cargo:rerun-if-changed=build.rs");

    // builder
    //     .files(&[
    //         "zsign/common/base64.cpp",
    //         "zsign/common/common.cpp",
    //         "zsign/common/json.cpp",
    //         "zsign/archo.cpp",
    //         "zsign/bundle.cpp",
    //         "zsign/macho.cpp",
    //         "zsign/main.cpp",
    //         "zsign/openssl.cpp",
    //         "zsign/signing.cpp",
    //         "zsign/zsign.cpp",
    //     ])
    //     .shared_flag(false)
    //     .compile("zsign");

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
    // println!("cargo:rustc-link-lib=dylib=zsign");
    // println!("cargo:rustc-link-search=native=/Users/lake/dounine/github/ipadump/zsign/build/macosx/arm64/release");
    // println!("cargo:rustc-link-search=native=./zsign/build/macosx/arm64/release/");

    // println!("cargo:rustc-link-lib=zsign"); //指定库
    // let bindings = bindgen::Builder::default()
    //     .header("./lib/main.h") //指定头文件，可以指定多个.h文件作为输入
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings");
    // bindings.write_to_file("./src/output.rs").unwrap(); //输出到那个目录

    // cd lib_static && xmake
    // println!("cargo:rustc-link-lib=static=hello_static");
    // println!("cargo:rustc-link-search=native=./lib_static/build/macosx/arm64/release/");
}