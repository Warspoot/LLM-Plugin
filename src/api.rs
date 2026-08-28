#![allow(dead_code)]

use std::ffi::{c_char, c_void, CStr};
use std::os::raw::c_int;
use std::sync::OnceLock;

pub enum Hachimi {}
pub enum Interceptor {}
pub enum Il2CppImage {}
pub enum Il2CppClass {}
pub enum FieldInfo {}
pub enum Il2CppObject {}
pub enum Il2CppString {}
pub enum MethodInfo {}
pub enum Il2CppThread {}

pub type HachimiGetApiFn = extern "C" fn(name: *const c_char) -> *mut c_void;
pub type HachimiInitV3Fn = extern "C" fn(get_api: HachimiGetApiFn, version: i32) -> InitResult;

#[repr(i32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InitResult {
    Error = 0,
    Ok = 1,
}

pub type GameInitializedCallback = unsafe extern "C" fn(userdata: *mut c_void);
pub type PresentCallback = unsafe extern "C" fn(swap_chain: *mut c_void, userdata: *mut c_void);

static GET_API: OnceLock<HachimiGetApiFn> = OnceLock::new();

pub fn set_get_api(f: HachimiGetApiFn) {
    let _ = GET_API.set(f);
}

fn get_api() -> HachimiGetApiFn {
    *GET_API.get().expect("api::set_get_api must run before any api::* call")
}

fn resolve<F: Copy>(name: &CStr) -> F {
    let p = (get_api())(name.as_ptr());
    assert!(!p.is_null(), "hachimi plugin API missing: {}", name.to_string_lossy());
    unsafe { std::mem::transmute_copy::<*mut c_void, F>(&p) }
}

macro_rules! api_fn {
    ($rust_name:ident, $c_name:literal, $ty:ty) => {
        pub fn $rust_name() -> $ty {
            static CELL: OnceLock<$ty> = OnceLock::new();
            *CELL.get_or_init(|| resolve::<$ty>(unsafe {
                CStr::from_bytes_with_nul_unchecked(concat!($c_name, "\0").as_bytes())
            }))
        }
    };
}

api_fn!(hachimi_instance, "hachimi_instance", unsafe extern "C" fn() -> *const Hachimi);
api_fn!(hachimi_get_interceptor, "hachimi_get_interceptor", unsafe extern "C" fn(*const Hachimi) -> *const Interceptor);
api_fn!(interceptor_hook, "interceptor_hook", unsafe extern "C" fn(*const Interceptor, *mut c_void, *mut c_void) -> *mut c_void);
api_fn!(interceptor_unhook, "interceptor_unhook", unsafe extern "C" fn(*const Interceptor, *mut c_void) -> *mut c_void);

api_fn!(il2cpp_get_assembly_image, "il2cpp_get_assembly_image", unsafe extern "C" fn(*const c_char) -> *const Il2CppImage);
api_fn!(il2cpp_get_class, "il2cpp_get_class", unsafe extern "C" fn(*const Il2CppImage, *const c_char, *const c_char) -> *mut Il2CppClass);
api_fn!(il2cpp_get_method_addr, "il2cpp_get_method_addr", unsafe extern "C" fn(*mut Il2CppClass, *const c_char, c_int) -> *mut c_void);
api_fn!(il2cpp_get_method_overload_addr, "il2cpp_get_method_overload_addr", unsafe extern "C" fn(*mut Il2CppClass, *const c_char, *const i32, usize) -> *mut c_void);
api_fn!(il2cpp_find_nested_class, "il2cpp_find_nested_class", unsafe extern "C" fn(*mut Il2CppClass, *const c_char) -> *mut Il2CppClass);
api_fn!(il2cpp_get_field_from_name, "il2cpp_get_field_from_name", unsafe extern "C" fn(*mut Il2CppClass, *const c_char) -> *mut FieldInfo);
api_fn!(il2cpp_get_field_value, "il2cpp_get_field_value", unsafe extern "C" fn(*mut Il2CppObject, *mut FieldInfo, *mut c_void));
api_fn!(il2cpp_set_field_value, "il2cpp_set_field_value", unsafe extern "C" fn(*mut Il2CppObject, *mut FieldInfo, *const c_void));
api_fn!(il2cpp_get_singleton_like_instance, "il2cpp_get_singleton_like_instance", unsafe extern "C" fn(*mut Il2CppClass) -> *mut Il2CppObject);
api_fn!(il2cpp_string_new, "il2cpp_string_new", unsafe extern "C" fn(*const c_char) -> *mut Il2CppString);
api_fn!(il2cpp_string_chars, "il2cpp_string_chars", unsafe extern "C" fn(*mut Il2CppString) -> *mut u16);
api_fn!(il2cpp_string_length, "il2cpp_string_length", unsafe extern "C" fn(*mut Il2CppString) -> i32);
api_fn!(il2cpp_get_main_thread, "il2cpp_get_main_thread", unsafe extern "C" fn() -> *mut Il2CppThread);
api_fn!(il2cpp_schedule_on_thread, "il2cpp_schedule_on_thread", unsafe extern "C" fn(*mut Il2CppThread, unsafe extern "C" fn()));

api_fn!(log_fn, "log", unsafe extern "C" fn(c_int, *const c_char, *const c_char));

api_fn!(hachimi_register_on_game_initialized, "hachimi_register_on_game_initialized", unsafe extern "C" fn(Option<GameInitializedCallback>, *mut c_void) -> bool);
api_fn!(hachimi_register_present_callback, "hachimi_register_present_callback", unsafe extern "C" fn(Option<PresentCallback>, *mut c_void) -> bool);
api_fn!(hachimi_get_base_dir, "hachimi_get_base_dir", unsafe extern "C" fn() -> *const c_char);
api_fn!(hachimi_get_data_path, "hachimi_get_data_path", unsafe extern "C" fn() -> *const c_char);

/// hachimi logging
pub fn log(level: log::Level, message: &str) {
    let lvl: c_int = match level {
        log::Level::Error => 1,
        log::Level::Warn => 2,
        log::Level::Info => 3,
        log::Level::Debug => 4,
        log::Level::Trace => 5,
    };
    let target = c"llm_plugin";
    let Ok(msg) = std::ffi::CString::new(message) else { return };
    unsafe { (log_fn())(lvl, target.as_ptr(), msg.as_ptr()) };
}
