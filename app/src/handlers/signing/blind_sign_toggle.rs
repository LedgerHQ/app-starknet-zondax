#[cfg(feature = "blind-sign-toggle")]
mod bindings {
    #[repr(C)]
    pub struct BlindSignToggle {
        pub toggle: bool,
        pub message: [i8; 9],
    }

    extern "C" {
        pub static mut blind_sign: BlindSignToggle;
    }
}

/// Returns if blind signing is enabled in this execution
pub fn blind_sign_enabled() -> bool {
    if cfg!(feature = "blind-sign-toggle") {
        //safe: guaranteed no data races
        unsafe { bindings::blind_sign.toggle }
    } else {
        true //specific request of this app, only blind signing
    }
}
