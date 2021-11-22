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
#![allow(unused_imports)]

pub use rslib::{
    constants::{self, ApduError, CLA},
    crypto::{self, Curve},
    rs_handle_apdu, PacketType,
};

pub use std::convert::TryInto;

pub use zemu_sys::set_out;

use bolos::crypto::bip32::BIP32Path;

pub fn handle_apdu(flags: &mut u32, tx: &mut u32, rx: u32, apdu_buffer: &mut [u8]) {
    unsafe {
        rs_handle_apdu(
            flags,
            tx,
            rx,
            apdu_buffer.as_mut_ptr(),
            apdu_buffer.len() as u16,
        )
    }
}

#[allow(dead_code)]
pub fn prepare_buffer<const LEN: usize>(
    buffer: &mut [u8; 260],
    path: &[u32],
    curve: Curve,
) -> usize {
    let crv: u8 = curve.into();
    let path = BIP32Path::<LEN>::new(path.iter().map(|n| 0x8000_0000 + n))
        .unwrap()
        .serialize();

    buffer[3] = crv;
    buffer[4] = path.len() as u8;
    buffer[5..5 + path.len()].copy_from_slice(path.as_slice());

    5 + path.len()
}

#[macro_export]
macro_rules! assert_error_code {
    ($tx:expr, $buffer:ident, $expected:expr) => {
        let pos: usize = $tx as _;
        let actual: ApduError = (&$buffer[pos - 2..pos]).try_into().unwrap();
        assert_eq!(actual, $expected);
    };
}
