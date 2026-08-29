use std::ffi::{c_void, CStr, CString};
use std::sync::Mutex;

use crate::api;
use crate::config;

struct Buffers {
    temperature: [u8; 32],
    top_k: [u8; 32],
    min_p: [u8; 32],
    repetition_penalty: [u8; 32],
}

fn fill(buf: &mut [u8], text: &str) {
    buf.fill(0);
    let bytes = text.as_bytes();
    let n = bytes.len().min(buf.len() - 1); // leave room for the trailing null byte
    buf[..n].copy_from_slice(&bytes[..n]);
}

fn buf_to_str(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}

impl Buffers {
    fn from_config(cfg: &config::Config) -> Self {
        let mut b = Buffers {
            temperature: [0; 32],
            top_k: [0; 32],
            min_p: [0; 32],
            repetition_penalty: [0; 32],
        };
        fill(&mut b.temperature, &format!("{:.3}", cfg.temperature));
        fill(&mut b.top_k, &cfg.top_k.to_string());
        fill(&mut b.min_p, &format!("{:.3}", cfg.min_p));
        fill(&mut b.repetition_penalty, &format!("{:.3}", cfg.repetition_penalty));
        b
    }
}

static BUFFERS: Mutex<Option<Buffers>> = Mutex::new(None);

fn f32_field(ui: *mut c_void, label: &CStr, buf: &mut [u8], value: &mut f32, min: f32, max: f32) -> bool {
    let mut changed = false;
    unsafe {
        (api::gui_ui_label())(ui, label.as_ptr());

        if (api::gui_ui_text_edit_singleline())(ui, buf.as_mut_ptr() as *mut _, buf.len()) {
            if let Ok(parsed) = buf_to_str(buf).parse::<f32>() {
                *value = parsed.clamp(min, max);
                fill(buf, &format!("{value:.3}"));
            }
            changed = true;
        }
    }
    changed
}

fn i32_field(ui: *mut c_void, label: &CStr, buf: &mut [u8], value: &mut i32, min: i32, max: i32) -> bool {
    let mut changed = false;
    unsafe {
        (api::gui_ui_label())(ui, label.as_ptr());

        if (api::gui_ui_text_edit_singleline())(ui, buf.as_mut_ptr() as *mut _, buf.len()) {
            if let Ok(parsed) = buf_to_str(buf).parse::<i32>() {
                *value = parsed.clamp(min, max);
                fill(buf, &value.to_string());
            }
            changed = true;
        }
    }
    changed
}

extern "C" fn section(ui: *mut c_void, _userdata: *mut c_void) {
    let mut cfg = config::get();
    let mut guard = BUFFERS.lock().unwrap();
    let buffers = guard.get_or_insert_with(|| Buffers::from_config(&cfg));

    let mut changed = false;

    unsafe {
        (api::gui_ui_heading())(ui, c"LLM Translation".as_ptr());
        (api::gui_ui_separator())(ui);

        if (api::gui_ui_checkbox())(ui, c"Enabled".as_ptr(), &mut cfg.enabled) {
            changed = true;
        }

        (api::gui_ui_separator())(ui);
    }

    if f32_field(ui, c"Temperature", &mut buffers.temperature, &mut cfg.temperature, 0.0, 2.0) {
        changed = true;
    }
    if i32_field(ui, c"Top K", &mut buffers.top_k, &mut cfg.top_k, 1, 100) {
        changed = true;
    }
    if f32_field(ui, c"Min P", &mut buffers.min_p, &mut cfg.min_p, 0.0, 1.0) {
        changed = true;
    }
    if f32_field(ui, c"Repetition penalty", &mut buffers.repetition_penalty, &mut cfg.repetition_penalty, 1.0, 2.0) {
        changed = true;
    }

    unsafe {
        (api::gui_ui_separator())(ui);
        (api::gui_ui_label())(ui, c"Endpoint, model, and prompts are edited in:".as_ptr());
        if let Some(name) = config::path().and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())) {
            if let Ok(name_c) = CString::new(name) {
                (api::gui_ui_label())(ui, name_c.as_ptr());
            }
        }

        if (api::gui_ui_button())(ui, c"Reload from file".as_ptr()) {
            let reloaded = config::reload();
            *guard = Some(Buffers::from_config(&reloaded));
            return;
        }
    }

    if changed {
        config::set(cfg);
    }
}

pub fn register() {
    let ok = unsafe { (api::gui_register_menu_section())(Some(section), std::ptr::null_mut()) };
    if ok {
        crate::logging::info("gui::register: menu section registered");
    } else {
        crate::logging::warn("gui::register: gui_register_menu_section failed");
    }
}
