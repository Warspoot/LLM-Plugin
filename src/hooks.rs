use std::ffi::CString;
use std::sync::OnceLock;

use crate::api;
use crate::api::Il2CppClass;

pub fn ready_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else { return 0 };
        let Ok(namespace) = CString::new("Gallop") else { return 0 };
        let Ok(name) = CString::new("ButtonCommon") else { return 0 };
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0;
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn install() {
    crate::logging::info("hooks::install: no hooks implemented yet");
}
