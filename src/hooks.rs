use std::ffi::CString;
use std::ffi::c_void;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

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

pub fn story_timeline_controller_class() -> *mut Il2CppClass {
    static CLASS: OnceLock<usize> = OnceLock::new();
    let addr = *CLASS.get_or_init(|| {
        let Ok(assembly) = CString::new("umamusume.dll") else {return 0};
        let Ok(namespace) = CString::new("Gallop") else {return 0};
        let Ok(name) = CString::new("StoryTimelineController") else {return 0};
        let image = unsafe { (api::il2cpp_get_assembly_image())(assembly.as_ptr()) };
        if image.is_null() {
            return 0
        }
        unsafe { (api::il2cpp_get_class())(image, namespace.as_ptr(), name.as_ptr()) as usize }
    });
    addr as *mut Il2CppClass
}

pub fn story_timeline_controller_initialize_addr() -> *mut c_void {
    static METHOD: OnceLock<usize> = OnceLock::new();
    let addr = *METHOD.get_or_init(|| {
        let class = story_timeline_controller_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("Initialize") else {return 0};
        unsafe { (api::il2cpp_get_method_addr())(class, name.as_ptr(), 0) as usize}
    });
    addr as *mut c_void
}

type StoryTimelineControllerInitializeFn = unsafe extern "C" fn (
    this: *mut api::Il2CppObject
);

static TRAMPOLINE: OnceLock<usize> = OnceLock::new();
static PENDING_CONTROLLER: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn story_timeline_controller_initialize_hook (
    this: *mut api::Il2CppObject
) {
    if let Some(&trampoline) = TRAMPOLINE.get() {
        let orig: StoryTimelineControllerInitializeFn = std::mem::transmute(trampoline);
        orig(this);
    }
    PENDING_CONTROLLER.store(this as usize, Ordering::Relaxed);
    crate::logging::info("story_timeline_controller_initialize_hook: fired");
}

pub fn install_story_timeline_controller_initialize_hook() {
    let target = story_timeline_controller_initialize_addr();
    if target.is_null() {
        crate::logging::warn("install_story_timeline_controller_initialize_hook: Initialize address not found, skipping hook");
        return;
    }

    let trampoline = unsafe {
        let hachimi = (api::hachimi_instance())();
        let interceptor = (api::hachimi_get_interceptor())(hachimi);
        (api::interceptor_hook())(interceptor, target, story_timeline_controller_initialize_hook as *mut c_void)
    };

    if trampoline.is_null() {
        crate::logging::warn("install_story_timeline_controller_initialize_hook: interceptor_hook failed");
        return;
    }

    let _ = TRAMPOLINE.set(trampoline as usize);
    crate::logging::info("install_story_timeline_controller_initialize_hook: hook installed")
}

pub fn get_timeline_data_addr() -> *mut c_void {
    static METHOD: OnceLock<usize> = OnceLock::new();
    let addr = *METHOD.get_or_init( || {
        let class = story_timeline_controller_class();
        if class.is_null() {
            return 0
        }
        let Ok(name) = CString::new("get_TimelineData") else {return 0};
        unsafe { (api::il2cpp_get_method_addr())(class, name.as_ptr(), 0) as usize}
    });
    addr as *mut c_void
}

type GetTimelineDataFn = unsafe extern "C" fn (
    this: *mut api::Il2CppObject
) -> *mut api::Il2CppObject;

unsafe fn get_timeline_data(this: *mut api::Il2CppObject) -> *mut api::Il2CppObject {
    let addr = get_timeline_data_addr();
    if addr.is_null() {return std::ptr::null_mut();}
    let f: GetTimelineDataFn = std::mem::transmute(addr);
    f(this)
}

pub fn poll() {
    let addr = PENDING_CONTROLLER.load(Ordering::Relaxed);
    if addr == 0 {
        return;
    }
    let this = addr as *mut api::Il2CppObject;
    let timeline_data = unsafe { get_timeline_data(this) };
    if timeline_data.is_null() {
        return; // nothing, check again next frame
    }
    PENDING_CONTROLLER.store(0, Ordering::Relaxed);
    crate::logging::info("hooks::poll: got TimelineData instance");
    crate::story::process(timeline_data);
}

pub fn install() {
    let class = story_timeline_controller_class();
    if class.is_null() {
        crate::logging::warn("hooks::install: StoryTimelineController class not found");
    }
    else {
        crate::logging::info("hooks::install: StoryTimelineController class found");
    }

    let initialize_addr = story_timeline_controller_initialize_addr();
    if initialize_addr.is_null() {
        crate::logging::warn("hooks::install: StoryTimelineController.Initialize method not found");
    }
    else {
        crate::logging::info("hooks::install: StoryTimelineController.Initialize method found");
    }
    install_story_timeline_controller_initialize_hook();

    crate::story::install();
}
