/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


pub const foo: _bindgen_ty_1 = _bindgen_ty_1::foo;
pub const bar: _bindgen_ty_1 = _bindgen_ty_1::bar;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 { foo = 4, bar = 8, }
pub type EasyToOverflow = ::std::os::raw::c_ulonglong;
pub const k: EasyToOverflow = 2147483648;
extern "C" {
    #[link_name = "\u{1}k_expr"]
    pub static k_expr: EasyToOverflow;
}
extern "C" {
    #[link_name = "\u{1}wow"]
    pub static wow: EasyToOverflow;
}
extern "C" {
    #[link_name = "\u{1}BAZ"]
    pub static BAZ: ::std::os::raw::c_longlong;
}
extern "C" {
    #[link_name = "\u{1}fuzz"]
    pub static fuzz: f64;
}
extern "C" {
    #[link_name = "\u{1}BAZZ"]
    pub static BAZZ: ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}WAT"]
    pub static WAT: ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}bytestring"]
    pub static mut bytestring: *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}NOT_UTF8"]
    pub static mut NOT_UTF8: *const ::std::os::raw::c_char;
}
