mod api;
mod cache;
mod config;
mod dictionary;
mod gui;
mod hooks;
mod il2cpp;
mod logging;
mod story;
mod llm;

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::time::Instant;

use api::InitResult;

static HOOKS_INSTALLED: AtomicBool = AtomicBool::new(false);

fn install_hooks_once() {
    if HOOKS_INSTALLED.swap(true, Ordering::Relaxed) {
        return; // already installed
    }
    let result = std::panic::catch_unwind(hooks::install);
    if result.is_err() {
        logging::error("hooks::install PANICKED (caught) - translation will not function this session");
    }
}

const SETTLE_MS: u128 = 5_000;
const READY_TIMEOUT_MS: u128 = 60_000;
static READY_CHECK_START: OnceLock<Instant> = OnceLock::new();

fn maybe_install_hooks() {
    if HOOKS_INSTALLED.load(Ordering::Relaxed) {
        return;
    }
    let start = *READY_CHECK_START.get_or_init(Instant::now);
    let elapsed = start.elapsed().as_millis();
    if elapsed < SETTLE_MS {
        return;
    }
    if elapsed < READY_TIMEOUT_MS && hooks::ready_class().is_null() {
        return; // nothing, try again next frame
    }
    logging::info(&format!("maybe_install_hooks: proceeding after {elapsed}ms"));
    install_hooks_once();
}

unsafe extern "C" fn on_present(_swap_chain: *mut c_void, _userdata: *mut c_void) {
    maybe_install_hooks();
    hooks::poll();
}

unsafe extern "C" fn on_game_initialized(_userdata: *mut c_void) {
    maybe_install_hooks();
}

#[no_mangle]
pub extern "C" fn hachimi_init_v3(get_api: api::HachimiGetApiFn, version: i32) -> InitResult {
    if version < 3 {
        return InitResult::Error;
    }

    api::set_get_api(get_api);
    logging::info(&format!("llm_plugin init (host plugin API v{version})"));

    let registered_present = unsafe {
        (api::hachimi_register_present_callback())(Some(on_present), std::ptr::null_mut())
    };
    if !registered_present {
        logging::warn("hachimi_register_present_callback failed - falling back to on_game_initialized only");
    }

    let registered_init = unsafe {
        (api::hachimi_register_on_game_initialized())(Some(on_game_initialized), std::ptr::null_mut())
    };
    if !registered_init {
        logging::warn("hachimi_register_on_game_initialized failed (harmless if the present callback registered)");
    }

    if !registered_present && !registered_init {
        logging::error("both hook-trigger registrations failed - plugin cannot install hooks");
        return InitResult::Error;
    }

    // gui shiiii
    let gui_result = std::panic::catch_unwind(gui::register);
    if gui_result.is_err() {
        logging::error("gui::register PANICKED (caught) - settings panel will not be available this session");
    }

    InitResult::Ok
}
