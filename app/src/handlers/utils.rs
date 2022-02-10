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

pub enum ConvertError<const R: usize = 32, const S: usize = 32> {
    /// The DER prefix (at index 0) found was different than the expected 0x30
    InvalidDERPrefix(u8),
    /// The R marker was different than expected (0x02)
    InvalidRMarker(u8),
    /// The encoded len for R was not the same as the expected
    InvalidRLen(usize),
    /// The S marker was different than expected (0x02)
    InvalidSMarker(u8),
    /// The encoded len for S was not the same as the expected
    InvalidSLen(usize),
    /// Passed signature was too short to be read properly
    TooShort,
    /// Passed signature encoded payload len was not in the expected range
    InvalidPayloadLen {
        min: usize,
        payload: usize,
        max: usize,
    },
}

#[inline(never)]
/// Converts a DER encoded signature into a RSV encoded signture
pub fn convert_der_to_rs<const R: usize, const S: usize>(
    sig: &[u8],
    out_r: &mut [u8; R],
    out_s: &mut [u8; S],
) -> Result<(), ConvertError<R, S>> {
    const MINPAYLOADLEN: usize = 1;
    const PAYLOADLEN: usize = 32;
    const MAXPAYLOADLEN: usize = 33;

    // https://github.com/libbitcoin/libbitcoin-system/wiki/ECDSA-and-DER-Signatures#serialised-der-signature-sequence
    // 0                [1 byte]   - DER Prefix
    // 1                [1 byte]   - Payload len
    // 2                [1 byte]   - R Marker. Always 02
    // 3                [1 byte]   - R Len                      RLEN
    // ROFFSET ...      [.?. byte] - R                          ROFFSET
    // ROFFSET+RLEN     [1 byte]   - S Marker. Always 02
    // ROFFSET+RLEN+1   [1 byte]   - S Length                   SLEN
    // ROFFSET+RLEN+2   [.?. byte] - S                          SOFFSET

    //check that we have at least the DER prefix and the payload len
    if sig.len() < 2 {
        return Err(ConvertError::TooShort);
    }

    //check DER prefix
    if sig[0] != 0x30 {
        return Err(ConvertError::InvalidDERPrefix(sig[0]));
    }

    //check payload len size
    let payload_len = sig[1] as usize;
    let min_payload_len = 2 + MINPAYLOADLEN + 2 + MINPAYLOADLEN;
    let max_payload_len = 2 + MAXPAYLOADLEN + 2 + MAXPAYLOADLEN;
    if payload_len < min_payload_len || payload_len > max_payload_len {
        return Err(ConvertError::InvalidPayloadLen {
            min: min_payload_len,
            payload: payload_len,
            max: max_payload_len,
        });
    }

    //check that the input slice is at least as long as the encoded len
    if sig.len() - 2 >= payload_len {
        return Err(ConvertError::TooShort);
    }

    //retrieve R
    if sig[2] != 0x02 {
        return Err(ConvertError::InvalidRMarker(sig[2]));
    }

    let r_len = sig[3] as usize;
    if r_len < MINPAYLOADLEN || r_len > MAXPAYLOADLEN {
        return Err(ConvertError::InvalidRLen(r_len));
    }

    //sig[4], after DER, after Payload, after marker after len
    let r = &sig[4..4 + r_len];

    //retrieve S
    if sig[4 + r_len] != 0x02 {
        return Err(ConvertError::InvalidSMarker(sig[4 + r_len]));
    }

    let s_len = sig[4 + r_len + 1] as usize;
    if s_len < MINPAYLOADLEN || s_len > MAXPAYLOADLEN {
        return Err(ConvertError::InvalidSLen(s_len));
    }

    //after r (4 + r_len), after marker, after len
    let s = &sig[4 + r_len + 2..4 + r_len + 2 + s_len];

    out_r.fill(0);
    out_r[PAYLOADLEN - r_len..].copy_from_slice(r);

    out_s.fill(0);
    out_s[PAYLOADLEN - s_len..].copy_from_slice(s);

    Ok(())
}
