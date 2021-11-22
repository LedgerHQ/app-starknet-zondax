/*******************************************************************************
*   (c) 2021 Zondax GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
#![no_std]
#![no_builtins]
#![macro_use]

extern crate no_std_compat as std;

pub mod constants;
pub mod dispatcher;
mod handlers;
mod sys;

pub use handlers::ZPacketType as PacketType;

#[cfg(not(fuzzing))]
sys::panic_handler! {}

#[macro_use]
mod utils;
use utils::ApduPanic;

pub mod crypto;

cfg_if::cfg_if! {
    if #[cfg(fuzzing)] {
        pub use dispatcher::handle_apdu;
    } else {
        use dispatcher::handle_apdu;
    }
}

use sys::{check_canary, zemu_log};

/// # Safety
///
/// This function is the app entry point for the minimal C stub
#[no_mangle]
pub unsafe extern "C" fn rs_handle_apdu(
    flags: *mut u32,
    tx: *mut u32,
    rx: u32,
    buffer: *mut u8,
    buffer_len: u16,
) {
    let flags = flags.as_mut().apdu_unwrap();
    let tx = tx.as_mut().apdu_unwrap();
    let data = std::slice::from_raw_parts_mut(buffer, buffer_len as usize);
    zemu_log("rs_handle_apdu\n\x00");

    handle_apdu(flags, tx, rx, data);

    check_canary();
}

#[cfg(test)]
pub fn handle_apdu_raw(bytes: &[u8]) -> (u32, u32, std::vec::Vec<u8>) {
    let mut flags = 0;
    let mut tx = 0;

    let rx = bytes.len();

    //prepare a big buffer for basically any output
    let mut out = std::vec![0; 0xFF];
    //copy input bytes
    out[..rx].copy_from_slice(bytes);

    //handle
    handle_apdu(&mut flags, &mut tx, rx as u32, &mut out);

    (flags, tx, out)
}
