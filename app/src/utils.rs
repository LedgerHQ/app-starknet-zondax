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
#![allow(dead_code)]

use bolos::PIC;

git_testament::git_testament_macros!(git);

pub const GIT_COMMIT_HASH: &str = git_commit_hash!();

mod apdu_wrapper;
pub use apdu_wrapper::*;

mod buffer_upload;
pub use buffer_upload::*;

#[cfg(test)]
#[macro_export]
macro_rules! assert_error_code {
    ($tx:expr, $buffer:ident, $expected:expr) => {
        let pos: usize = $tx as _;
        let actual: ApduError = (&$buffer[pos - 2..pos]).try_into().unwrap();
        assert_eq!(actual, $expected);
    };
}

/// This function returns the index of the first null byte in the slice
#[cfg(test)]
pub fn strlen(s: &[u8]) -> usize {
    let mut count = 0;
    while let Some(&c) = s.get(count) {
        if c == 0 {
            return count;
        }
        count += 1;
    }

    panic!("byte slice did not terminate with null byte, s: {:x?}", s)
}

pub struct OutputBufferTooSmall;
pub fn hex_encode(
    input: impl AsRef<[u8]>,
    output: &mut [u8],
) -> Result<usize, OutputBufferTooSmall> {
    let input = input.as_ref();

    if input.len() * 2 > output.len() {
        return Err(OutputBufferTooSmall);
    }

    const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";

    let table = PIC::new(HEX_CHARS_LOWER).into_inner();
    for (byte, out) in input.iter().zip(output.chunks_mut(2)) {
        let high = table[((byte & 0xf0) >> 4) as usize];
        let low = table[(byte & 0xf) as usize];

        //number of items guaranteed
        // as we checked the size beforehand so
        // output will always be at least the right length
        // to encode input
        out[0] = high;
        out[1] = low;
    }

    Ok(input.len() * 2)
}

#[cfg(test)]
mod maybe_null_terminated_to_string {
    use core::str::Utf8Error;
    use std::borrow::ToOwned;
    use std::ffi::{CStr, CString};
    use std::string::String;

    ///This trait is a utility trait to convert a slice of bytes into a CString
    ///
    /// If the string is nul terminated already then no null termination is added
    pub trait MaybeNullTerminatedToString {
        fn to_string_with_check_null(&self) -> Result<String, Utf8Error>;
    }

    impl MaybeNullTerminatedToString for &[u8] {
        fn to_string_with_check_null(&self) -> Result<String, Utf8Error> {
            //attempt to make a cstr first
            if let Ok(cstr) = CStr::from_bytes_with_nul(self) {
                return cstr.to_owned().into_string().map_err(|e| e.utf8_error());
            }

            //in the case above,
            // we could be erroring due to a null byte in the middle
            // or a null byte _missing_ at the end
            //
            //but here we'll error for a null byte at the end or a null byte in the middle
            match CString::new(self.to_vec()) {
                Ok(cstring) => cstring.into_string().map_err(|e| e.utf8_error()),
                Err(err) => {
                    // so with the above error, we can only be erroring here only with a null byte in the middle
                    let nul_pos = err.nul_position();
                    //truncate the string
                    CStr::from_bytes_with_nul(&self[..=nul_pos])
                        //we can't be erroring for a missing null byte at the end,
                        // and also can't error due to a null byte in the middle,
                        // because this is literally the smaller substring to be terminated
                        .unwrap()
                        .to_owned()
                        .into_string()
                        .map_err(|e| e.utf8_error())
                }
            }
        }
    }

    impl<const S: usize> MaybeNullTerminatedToString for [u8; S] {
        fn to_string_with_check_null(&self) -> Result<String, Utf8Error> {
            (&self[..]).to_string_with_check_null()
        }
    }
}

#[cfg(test)]
pub use maybe_null_terminated_to_string::MaybeNullTerminatedToString;

pub trait ApduPanic: Sized {
    type Item;

    fn apdu_unwrap(self) -> Self::Item;

    fn apdu_expect(self, s: &str) -> Self::Item;
}

impl<T, E> ApduPanic for Result<T, E> {
    type Item = T;

    #[inline]
    fn apdu_unwrap(self) -> Self::Item {
        match self {
            Ok(t) => t,
            Err(_) => panic!(),
        }
    }

    #[inline]
    fn apdu_expect(self, _: &str) -> Self::Item {
        match self {
            Ok(t) => t,
            Err(_) => panic!(),
        }
    }
}
impl<T> ApduPanic for Option<T> {
    type Item = T;

    #[inline]
    fn apdu_unwrap(self) -> Self::Item {
        match self {
            Some(t) => t,
            None => panic!(),
        }
    }

    #[inline]
    fn apdu_expect(self, _: &str) -> Self::Item {
        match self {
            Some(t) => t,
            None => panic!(),
        }
    }
}
