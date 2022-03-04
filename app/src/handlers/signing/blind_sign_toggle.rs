#[cfg(feature = "blind-sign-toggle")]
mod impls {
    #[repr(C)]
    pub struct BlindSignToggle {
        pub toggle: bool,
        pub message: [i8; 9],
    }

    cfg_if::cfg_if! {
        if #[cfg(any(unix, windows))] {
            /// Provide a mock for tests
            #[allow(non_upper_case_globals)]
            pub static mut blind_sign: BlindSignToggle = BlindSignToggle {
                toggle: true,
                message: [0; 9],
            };
        } else {
            extern "C" {
                ///Link to the C code
                pub static mut blind_sign: BlindSignToggle;
            }
        }
    }
}

/// Returns if blind signing is enabled in this execution
pub fn blind_sign_enabled() -> bool {
    if cfg!(feature = "blind-sign-toggle") {
        //safe: guaranteed no data races
        unsafe { impls::blind_sign.toggle }
    } else {
        true //specific request of this app, only blind signing
    }
}
