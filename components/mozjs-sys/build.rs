fn main() {
    println!("cargo:rerun-if-env-changed=LIB");

    // 设置库文件路径（根据实际情况调整路径）
    println!("cargo:rustc-link-search=native=D:\\servo2025\\components\\mozjs-sys\\src");

    // 指定要链接的静态库文件
    // println!("cargo:rustc-link-lib=static=rusty_v8");
}