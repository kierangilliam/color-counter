extern crate web_sys;

use std::convert::TryInto;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn time(label: &str) {
    web_sys::console::time_with_label(label);
}

pub fn timeEnd(label: &str) {
    web_sys::console::time_end_with_label(label);
}

pub fn rgb(rgba: &[u8]) -> [u8; 3] {
    rgba[..3].try_into().expect("slice with incorrect length")
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
