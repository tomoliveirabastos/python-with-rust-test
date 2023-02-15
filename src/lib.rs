use serde_json::{json, Value};
use std::ffi::{CStr, CString};
mod pessoa;

use pessoa::pessoa::Pessoa;

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[no_mangle]
pub extern "C" fn say_my_name(name: &CString) -> &CString {
    return name;
}

#[no_mangle]
pub extern "C" fn handle_json(ptr: *const libc::c_char) {
    let cstr = unsafe { CStr::from_ptr(ptr) };

    let a = cstr.to_str().unwrap();

    let json_value: Value = serde_json::from_str(a).expect("");

    let nome = json_value
        .get("nome")
        .unwrap()
        .to_owned()
        .as_str()
        .unwrap()
        .to_string();

    let idade = json_value
        .get("idade")
        .unwrap()
        .to_owned()
        .as_i64()
        .unwrap();

    let pessoa: Pessoa<i64> = Pessoa::new(nome, idade);

    println!("{:?}", pessoa);
}

#[no_mangle]
pub extern "C" fn pessoa_to_json(nome: *const libc::c_char, idade: i32) -> CString {
    let cstr = unsafe { CStr::from_ptr(nome) };

    let nome2 = cstr.to_str().unwrap().to_string();

    let pessoa = Pessoa::new(nome2, idade);

    let json = json!(pessoa).to_string();

    let c_json = CString::new(json).expect("falhou");

    return c_json;
}
