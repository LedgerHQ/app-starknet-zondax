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
use bolos::nvm::NVMError;

use crate::{
    constants::ApduError,
    handlers::{
        lock::LockError,
        resources::{BUFFERAccessors, BUFFER},
        ZPacketType,
    },
};

use super::ApduBufferRead;

#[bolos::lazy_static]
static mut INIT_LEN: usize = 0;

pub struct Uploader {
    accessor: BUFFERAccessors,
}

pub enum UploaderError {
    /// Couldn't parse PacketType
    PacketTypeParseError,

    /// PacketType wasn't init, next or last
    PacketTypeInvalid,

    /// Error with `BUFFER` lock
    Lock(LockError),

    /// Error writing to `BUFFER`
    Nvm(NVMError),
}

impl From<LockError> for UploaderError {
    fn from(e: LockError) -> Self {
        Self::Lock(e)
    }
}

impl From<NVMError> for UploaderError {
    fn from(e: NVMError) -> Self {
        Self::Nvm(e)
    }
}

impl From<UploaderError> for ApduError {
    fn from(e: UploaderError) -> Self {
        match e {
            UploaderError::PacketTypeInvalid | UploaderError::PacketTypeParseError => {
                ApduError::InvalidP1P2
            }
            UploaderError::Nvm(_) => ApduError::DataInvalid,
            UploaderError::Lock(e) => e.into(),
        }
    }
}

pub struct UploaderOutput {
    pub p2: u8,
    pub first: &'static [u8],
    pub data: &'static [u8],
    accessor: BUFFERAccessors,
}

impl Drop for UploaderOutput {
    fn drop(&mut self) {
        unsafe {
            if let Ok(zbuffer) = BUFFER.acquire(self.accessor) {
                zbuffer.reset();

                //we managed to acquire so we should release too
                let _ = BUFFER.release(self.accessor);
            }

            //couldn't acquire BUFFER so someone is trying to use it
        }
    }
}

impl Uploader {
    pub fn new(accessor: impl Into<BUFFERAccessors>) -> Self {
        Self {
            accessor: accessor.into(),
        }
    }

    #[inline(never)]
    pub fn upload(
        &mut self,
        buffer: &ApduBufferRead<'_>,
    ) -> Result<Option<UploaderOutput>, UploaderError> {
        let packet_type =
            ZPacketType::new(buffer.p1()).map_err(|_| UploaderError::PacketTypeParseError)?;

        if packet_type.is_init() {
            let zbuffer = unsafe { BUFFER.lock(self.accessor)? };
            zbuffer.reset();

            zbuffer.write(&[buffer.p2()])?;
            if let Ok(payload) = buffer.payload() {
                unsafe {
                    *INIT_LEN = payload.len();
                }
                zbuffer.write(payload)?;
            }

            Ok(None)
        } else if packet_type.is_next() {
            let zbuffer = unsafe { BUFFER.acquire(self.accessor)? };

            if let Ok(payload) = buffer.payload() {
                zbuffer.write(payload)?;
            }

            Ok(None)
        } else if packet_type.is_last() {
            let zbuffer = unsafe { BUFFER.acquire(self.accessor)? };

            if let Ok(payload) = buffer.payload() {
                zbuffer.write(payload)?;
            }

            let data = zbuffer.read_exact();
            let (head, tail) = data[1..].split_at(unsafe { *INIT_LEN });

            Ok(Some(UploaderOutput {
                p2: data[0],
                first: head,
                data: tail,
                accessor: self.accessor,
            }))
        } else {
            Err(UploaderError::PacketTypeInvalid)
        }
    }
}
