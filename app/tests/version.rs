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

use constants::{version::*, INS_GET_VERSION as INS};

#[test]
fn version() {
    let mut flags = 0u32;
    let mut tx = 0u32;
    let rx = 5;
    let mut buffer = [0u8; 260];

    buffer[..3].copy_from_slice(&[CLA, INS, 0]);

    handle_apdu(&mut flags, &mut tx, rx, &mut buffer);

    //debug mode, [M, N, P], ux allowed, target_id, result code
    assert_eq!(tx, 1 + 3 + 1 + 4 + 2);
    assert_error_code!(tx, buffer, ApduError::Success);

    assert_eq!(buffer[1], APPVERSION_M);
    assert_eq!(buffer[2], APPVERSION_N);
    assert_eq!(buffer[3], APPVERSION_P);
}
