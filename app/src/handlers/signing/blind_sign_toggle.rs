#[repr(C)]
pub struct BlindSignToggle {
    pub toggle: bool,
    pub message: [i8; 9],
}

extern "C" {
    pub static mut blind_sign: BlindSignToggle;
}
