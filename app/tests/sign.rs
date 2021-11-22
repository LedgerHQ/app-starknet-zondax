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
mod prelude;
use prelude::*;

use bolos::hash::{Hasher, Sha256};
use constants::INS_SIGN as INS;

const MSG: &[u8] = b"hello@zondax.ch";

#[test]
#[ignore] //`set_out` doesn't work properly
fn sign() {
    let mut flags = 0;
    let mut tx = 0;
    let mut buffer = [0; 260];

    buffer[0] = CLA;
    buffer[1] = INS;
    buffer[2] = PacketType::Init.into();
    let len = prepare_buffer::<4>(&mut buffer, &[44, 0, 0, 0], Curve::Ed25519);

    handle_apdu(&mut flags, &mut tx, 5 + len as u32, &mut buffer);
    assert_error_code!(tx, buffer, ApduError::Success);

    buffer[0] = CLA;
    buffer[1] = INS;
    buffer[2] = PacketType::Last.into();
    buffer[3] = 0;
    buffer[4] = MSG.len() as u8;
    buffer[5..5 + MSG.len()].copy_from_slice(MSG);

    set_out(&mut buffer);
    handle_apdu(&mut flags, &mut tx, 5 + MSG.len() as u32, &mut buffer);
    assert_error_code!(tx, buffer, ApduError::Success);

    let out_hash = &buffer[..32];
    let expected = Sha256::digest(MSG).unwrap();
    assert_eq!(&expected, out_hash);
}
