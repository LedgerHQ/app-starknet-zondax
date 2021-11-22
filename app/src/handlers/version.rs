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
use crate::constants::{version::*, ApduError};
use crate::dispatcher::ApduHandler;
use crate::utils::ApduBufferRead;

pub struct GetVersion {}

pub fn get_target_id() -> Result<u32, ApduError> {
    Ok(0u32)
}

impl ApduHandler for GetVersion {
    #[inline(never)]
    fn handle<'apdu>(
        _: &mut u32,
        tx: &mut u32,
        apdu_buffer: ApduBufferRead<'apdu>,
    ) -> Result<(), ApduError> {
        crate::sys::zemu_log_stack("GetVersion\x00");
        *tx = 0;

        let apdu_buffer = apdu_buffer.write();
        apdu_buffer[0] = 0; //Debug mode
                            // Version
        apdu_buffer[1] = APPVERSION_M;
        apdu_buffer[2] = APPVERSION_N;
        apdu_buffer[3] = APPVERSION_P;
        apdu_buffer[4] = 0; //UX allowed

        // target id
        let target_id_slice = get_target_id()?.to_be_bytes();
        apdu_buffer[5..9].clone_from_slice(&target_id_slice);
        *tx = 9;

        Ok(())
    }
}
