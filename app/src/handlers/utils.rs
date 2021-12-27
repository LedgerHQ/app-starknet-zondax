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
use crate::{
    constants::{ApduError, STARK_BIP32_PATH_0, STARK_BIP32_PATH_1},
    sys::{crypto::bip32::BIP32Path, ViewError, PIC},
};

use core::convert::TryFrom;

#[repr(u8)]
pub enum ZPacketType {
    Init = 0,
    Add = 1,
    Last = 2,
}

impl std::convert::TryFrom<u8> for ZPacketType {
    type Error = ();

    fn try_from(from: u8) -> Result<Self, ()> {
        match from {
            0 => Ok(Self::Init),
            1 => Ok(Self::Add),
            2 => Ok(Self::Last),
            _ => Err(()),
        }
    }
}

impl From<ZPacketType> for u8 {
    fn from(from: ZPacketType) -> Self {
        from as _
    }
}

impl ZPacketType {
    #[allow(clippy::result_unit_err)]
    pub fn new(p1: u8) -> Result<Self, ()> {
        Self::try_from(p1)
    }

    pub fn is_init(&self) -> bool {
        matches!(self, Self::Init)
    }

    pub fn is_last(&self) -> bool {
        matches!(self, Self::Last)
    }

    pub fn is_next(&self) -> bool {
        !self.is_init() && !self.is_last()
    }
}

#[inline(never)]
pub fn handle_ui_message(item: &[u8], out: &mut [u8], page: u8) -> Result<u8, ViewError> {
    crate::sys::zemu_log_stack("handle_ui_message\x00");
    let m_len = out.len() - 1; //null byte terminator
    if m_len <= item.len() {
        let chunk = item
            .chunks(m_len) //divide in non-overlapping chunks
            .nth(page as usize) //get the nth chunk
            .ok_or(ViewError::Unknown)?;

        out[..chunk.len()].copy_from_slice(chunk);
        out[chunk.len()] = 0; //null terminate

        let n_pages = item.len() / m_len;
        Ok(1 + n_pages as u8)
    } else {
        out[..item.len()].copy_from_slice(item);
        out[item.len()] = 0; //null terminate
        Ok(1)
    }
}

#[inline(never)]
///Verify path starts with the stark-specific derivation path
pub fn verify_bip32_path<const B: usize>(path: &BIP32Path<B>) -> Result<(), ApduError> {
    let path_0 = *PIC::new(STARK_BIP32_PATH_0).get_ref();
    let path_1 = *PIC::new(STARK_BIP32_PATH_1).get_ref();

    //verify path starts with the stark-specific derivation path
    if !path.components().starts_with(&[path_0, path_1]) {
        Err(ApduError::DataInvalid)
    } else {
        Ok(())
    }
}
