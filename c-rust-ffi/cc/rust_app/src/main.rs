use std::os::raw::c_int;

unsafe extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn hello_from_c();
}

fn main() {
    unsafe {
        let result = add(3, 4);
        println!("3 + 4 = {}", result);
        hello_from_c();
    }
}
