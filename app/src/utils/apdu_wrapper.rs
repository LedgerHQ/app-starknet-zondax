use crate::constants::{
    APDU_INDEX_CLA, APDU_INDEX_INS, APDU_INDEX_LEN, APDU_INDEX_P1, APDU_INDEX_P2, APDU_MIN_LENGTH,
};

/// Wraps an apdu_buffer and provides utility methods
pub struct ApduBufferRead<'apdu> {
    inner: &'apdu mut [u8],
}

#[derive(PartialEq)]
pub enum ApduBufferReadError {
    /// The provided buffer was not long enough
    ///
    /// This happens if the slice is less than it should be for minimum
    /// or if the provided slice is too short for the provided `rx`
    LengthMismatch { expected: usize, got: usize },

    /// The requested payload was too short then expected
    NotEnoughPayload { expected: usize, got: usize },

    /// The provided buffer was too short and didn't have a payload
    NoPayload,
}

impl ApduBufferReadError {
    pub fn length_to_payload(self) -> Self {
        match self {
            Self::LengthMismatch { expected, got } => Self::NotEnoughPayload { expected, got },
            err => err,
        }
    }
}

impl<'apdu> ApduBufferRead<'apdu> {
    fn check_min_len(
        len: usize,
        expected: usize,
        offset: impl Into<Option<usize>>,
    ) -> Result<(), ApduBufferReadError> {
        let offset = offset.into().unwrap_or_default();

        if len - offset < expected {
            Err(ApduBufferReadError::LengthMismatch {
                expected,
                got: len - offset,
            })
        } else {
            Ok(())
        }
    }

    /// Create a new "ApduBuffer" from the given mutable byte slice
    ///
    /// The function checks if there's at least the minimum required number of bytes (APDU_MIN_LENGTH)
    /// and if the byte slice is at least as long as rx
    #[inline(never)]
    pub fn new(buf: &'apdu mut [u8], rx: u32) -> Result<Self, ApduBufferReadError> {
        crate::sys::zemu_log_stack("ApduBufferRead::new\x00");
        //check buf is at least 4
        Self::check_min_len(buf.len(), APDU_MIN_LENGTH as usize, None)?;

        //check rx is at least 4
        Self::check_min_len(rx as usize, APDU_MIN_LENGTH as usize, None)?;

        //check buf is at least rx
        Self::check_min_len(buf.len(), rx as usize, None)?;

        Ok(Self { inner: buf })
    }

    /// Alias to idx APDU_INDEX_CLA
    pub fn cla(&self) -> u8 {
        self.inner[APDU_INDEX_CLA]
    }

    /// Alias to idx APDU_INDEX_INS
    pub fn ins(&self) -> u8 {
        self.inner[APDU_INDEX_INS]
    }

    /// Alias to idx APDU_INDEX_P1
    pub fn p1(&self) -> u8 {
        self.inner[APDU_INDEX_P1]
    }

    /// Alias to idx APDU_INDEX_P2
    pub fn p2(&self) -> u8 {
        self.inner[APDU_INDEX_P2]
    }

    /// Return the remaining part of the buffer if present
    ///
    /// It's expected the buffer to have the prepended len at idx APDU_INDEX_LEN,
    /// thus the data would start at idx 5 until len - 5
    pub fn payload(&self) -> Result<&[u8], ApduBufferReadError> {
        let plen = self.inner[APDU_INDEX_LEN] as usize;
        //check that the buffer is long enough for the payload
        Self::check_min_len(self.inner.len(), plen as usize, APDU_MIN_LENGTH as usize)
            .map_err(|err| err.length_to_payload())?;

        Ok(&self.inner[APDU_MIN_LENGTH as usize..APDU_MIN_LENGTH as usize + plen])
        //we checked the size beforehand
    }

    /// Discard the structure to obtain the inner slice for writing
    pub fn write(self) -> &'apdu mut [u8] {
        self.inner
    }
}
