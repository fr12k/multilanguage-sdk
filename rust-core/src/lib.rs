uniffi::setup_scaffolding!();
use uniffi::export;
use ffi_support::{rust_string_to_c};

#[export]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[no_mangle]
#[export]
pub extern fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[export]
pub extern fn http() -> String{
    let res = client();
    res
}

#[no_mangle]
pub extern fn gohttp() -> *mut std::os::raw::c_char {

    let res = client();

    // Convert the response into a C-compatible string
    rust_string_to_c(res)
}

pub fn client() -> String {
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://www.rust-lang.org").send().unwrap();
    res.text().unwrap()
}
