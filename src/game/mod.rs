pub use self::poker::*;
pub use self::slots::*;

mod poker;
mod slots;

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
use js_sys::Date;

#[cfg(not(target_arch = "wasm32"))]
pub fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis() as f64 / 1000.0
}
#[cfg(target_arch = "wasm32")]
pub fn current_time() -> f64 {
    Date::now() / 1000.0
}
