fn main() {
    cc::Build::new()
        .file("../c_lib/hello.c")
        .include("../c_lib")
        .compile("hello");
}
