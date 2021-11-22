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

use constants::INS_GET_PUBLIC_KEY as INS;

#[test]
fn public_key() {
    let mut flags = 0u32;
    let mut tx = 0u32;
    let rx = 5;
    let mut buffer = [0u8; 260];

    buffer[..3].copy_from_slice(&[CLA, INS, 0]);
    prepare_buffer::<4>(&mut buffer, &[44, 0, 0, 0], Curve::Ed25519);

    handle_apdu(&mut flags, &mut tx, rx, &mut buffer);

    assert_error_code!(tx, buffer, ApduError::Success);
    assert_eq!(tx as usize, 1 + 32 + 2 + 32); //32 bytes for ed25519 and 32 bytes for hash
}
