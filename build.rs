extern crate cc;

fn main() {
    let mut builder: cc::Build = cc::Build::new();
    builder
        .file("src/c_code/library.c")
        .shared_flag(false)
        .compile("hello_for_static");

    //搜索动态库的路径，这里是相对路径，相对于build.rs所在的目录
    //可同时支持静态跟动态库
    //请先去lib文件夹下面执行make命令，生成动态库
    //```bash
    // cd lib
    // mkdir build && cd build
    // cmake ..
    // make
    // ```
    println!("cargo:rustc-link-lib=dylib=hello");
    println!("cargo:rustc-link-search=native=./lib/build");
}