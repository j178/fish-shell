/// Bridged functions concerned with initialization.
use crate::ffi::wcharz_t;
use crate::locale;

#[cxx::bridge]
mod ffi2 {

    extern "C++" {
        include!("wutil.h");
        type wcharz_t = super::wcharz_t;
    }

    extern "Rust" {
        fn rust_init();
        fn rust_activate_flog_categories_by_pattern(wc_ptr: wcharz_t);
        fn rust_invalidate_numeric_locale();
    }
}

/// Entry point for Rust-specific initialization.
fn rust_init() {
    crate::topic_monitor::topic_monitor_init();
    crate::future_feature_flags::future_feature_flags_init();
    crate::threads::init();
}

/// FFI bridge for activate_flog_categories_by_pattern().
fn rust_activate_flog_categories_by_pattern(wc_ptr: wcharz_t) {
    crate::flog::activate_flog_categories_by_pattern(wc_ptr.into());
}

/// FFI bridge to invalidate cached locale bits.
fn rust_invalidate_numeric_locale() {
    locale::invalidate_numeric_locale();
}
