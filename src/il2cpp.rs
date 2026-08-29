use std::ffi::c_void;
use crate::api;

pub unsafe fn list_len(list: *mut c_void) -> i32 {
    if list.is_null() {
        return 0;
    }
    ((list as usize + 0x18) as *const i32).read_unaligned()
}

pub unsafe fn list_ref_at(list: *mut c_void, i: i32) -> *mut c_void {
    if list.is_null() || i < 0 {
        return std::ptr::null_mut();
    }
    let items = ((list as usize + 0x10) as *const *mut c_void).read_unaligned();
    if items.is_null() {
        return std::ptr::null_mut();
    }
    ((items as usize + 0x20 + i as usize * 8) as *const *mut c_void).read_unaligned()
}

pub unsafe fn read_il2cpp_string(s: *mut api::Il2CppString) -> String {
    if s.is_null() {
        return String::new();
    }
    let len = (api::il2cpp_string_length())(s);
    if len <= 0 {
        return String::new();
    }
    let chars = (api::il2cpp_string_chars())(s);
    if chars.is_null() {
        return String::new();
    }
    String::from_utf16_lossy(std::slice::from_raw_parts(chars, len as usize))
}
