use std::os::raw::c_int;

unsafe extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn hello_from_c();
    fn hello_from_c_shared();
}

fn main() {
    unsafe {
        let result = add(10, 20);
        println!("10 + 20 = {}", result);

        hello_from_c();

        // `export LD_LIBRARY_PATH=../c_lib_shared` is required
        hello_from_c_shared();
    }
}
