fn main() {
    // static
    println!("cargo:rustc-link-search=native=../c_lib");
    println!("cargo:rustc-link-lib=static=hello");

    // shared
    println!("cargo:rustc-link-search=native=../c_lib_shared");
    println!("cargo:rustc-link-lib=dylib=hello_shared");
}
