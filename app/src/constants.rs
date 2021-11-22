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
// Based on ISO7816
#[repr(u16)]
#[derive(PartialEq, Eq)]
#[cfg_attr(any(test, feature = "derive-debug"), derive(Debug))]
pub enum ApduError {
    ExecutionError = 0x6400,
    WrongLength = 0x6700,
    ApduCodeEmptyBuffer = 0x6982,
    OutputBufferTooSmall = 0x6983,
    DataInvalid = 0x6984,
    ApduCodeConditionsNotSatisfied = 0x6985,
    CommandNotAllowed = 0x6986,
    BadKeyExample = 0x6A80,
    InvalidP1P2 = 0x6B00,
    InsNotSupported = 0x6D00,
    ClaNotSupported = 0x6E00,
    Unknown = 0x6F00,
    SignVerifyError = 0x6F01,
    Success = 0x9000,
    Busy = 0x9001,
}

#[cfg_attr(any(test, feature = "derive-debug"), derive(Debug))]
pub enum ConvertApduError {
    Length { expected: usize, found: usize },
    Unknown(u16),
}

impl std::convert::TryFrom<&[u8]> for ApduError {
    type Error = ConvertApduError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > 2 {
            return Err(Self::Error::Length {
                expected: 2,
                found: value.len(),
            });
        }

        let value = {
            let mut array = [0; 2];
            array.copy_from_slice(value);
            u16::from_be_bytes(array)
        };

        Self::try_from(value)
    }
}

impl std::convert::TryFrom<u16> for ApduError {
    type Error = ConvertApduError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x6400 => Ok(Self::ExecutionError),
            0x6700 => Ok(Self::WrongLength),
            0x6982 => Ok(Self::ApduCodeEmptyBuffer),
            0x6983 => Ok(Self::OutputBufferTooSmall),
            0x6984 => Ok(Self::DataInvalid),
            0x6985 => Ok(Self::ApduCodeConditionsNotSatisfied),
            0x6986 => Ok(Self::CommandNotAllowed),
            0x6A80 => Ok(Self::BadKeyExample),
            0x6B00 => Ok(Self::InvalidP1P2),
            0x6D00 => Ok(Self::InsNotSupported),
            0x6E00 => Ok(Self::ClaNotSupported),
            0x6F00 => Ok(Self::Unknown),
            0x6F01 => Ok(Self::SignVerifyError),
            0x9000 => Ok(Self::Success),
            0x9001 => Ok(Self::Busy),
            err => Err(Self::Error::Unknown(err)),
        }
    }
}

pub const APDU_INDEX_CLA: usize = 0;
pub const APDU_INDEX_INS: usize = 1;
pub const APDU_INDEX_P1: usize = 2;
pub const APDU_INDEX_P2: usize = 3;
pub const APDU_INDEX_LEN: usize = 4;

pub const APDU_MIN_LENGTH: u32 = 5;

pub const EDWARDS_SIGN_BUFFER_MIN_LENGTH: usize = 64;

pub(crate) mod instructions {
    pub const CLA: u8 = 0xFF;

    pub const INS_GET_VERSION: u8 = 0x00;
    pub const INS_GET_PUBLIC_KEY: u8 = 0x01;
    pub const INS_SIGN: u8 = 0x02;
}

pub use instructions::*;

pub mod version {
    template_app_derive::version!("Makefile.version");
}
