pub fn error(msg: &str) { crate::api::log(log::Level::Error, msg); }
pub fn warn(msg: &str) { crate::api::log(log::Level::Warn, msg); }
pub fn info(msg: &str) { crate::api::log(log::Level::Info, msg); }
pub fn debug(msg: &str) { crate::api::log(log::Level::Debug, msg); }
