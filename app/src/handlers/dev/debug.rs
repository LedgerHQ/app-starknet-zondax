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
use std::prelude::v1::*;

use crate::{
    constants::ApduError as Error,
    dispatcher::ApduHandler,
    handlers::{handle_ui_message, resources::BUFFER},
    sys::{Show, ViewError, Viewable, PIC},
    utils::ApduBufferRead,
};

#[derive(Default)]
pub struct Debug;

impl ApduHandler for Debug {
    #[inline(never)]
    fn handle<'apdu>(
        flags: &mut u32,
        tx: &mut u32,
        apdu: ApduBufferRead<'apdu>,
    ) -> Result<(), Error> {
        *tx = 0;

        let payload = apdu.payload().map_err(|_| Error::DataInvalid)?;

        let zbuffer = unsafe { BUFFER.lock(Self)? };
        zbuffer
            .write(&[
                apdu.cla(),
                apdu.ins(),
                apdu.p1(),
                apdu.p2(),
                payload.len() as u8,
            ])
            .map_err(|_| Error::ExecutionError)?;
        zbuffer.write(payload).map_err(|_| Error::ExecutionError)?;

        unsafe { Self.show(flags).map_err(|_| Error::ExecutionError) }
    }
}

impl Debug {
    fn cleanup(&mut self) {
        unsafe {
            if let Ok(zbuffer) = BUFFER.acquire(Self) {
                zbuffer.reset();

                //we managed to acquire so we should release too
                let _ = BUFFER.release(Self);
            }

            //couldn't acquire BUFFER so someone is trying to use it
        }
    }

    fn get_buf() -> Result<&'static [u8], Error> {
        let zbuffer = unsafe { BUFFER.acquire(Self).map_err(|_| Error::ExecutionError)? };
        Ok(zbuffer.read_exact())
    }
}

impl Viewable for Debug {
    fn num_items(&mut self) -> Result<u8, ViewError> {
        Ok(1)
    }

    fn render_item(
        &mut self,
        idx: u8,
        title: &mut [u8],
        message: &mut [u8],
        page: u8,
    ) -> Result<u8, ViewError> {
        if let 0 = idx {
            title[..5].copy_from_slice(&PIC::new(b"APDU\x00").into_inner()[..]);

            handle_ui_message(
                Self::get_buf().map_err(|_| ViewError::Unknown)?,
                message,
                page,
            )
        } else {
            Err(ViewError::NoData)
        }
    }

    fn accept(&mut self, _: &mut [u8]) -> (usize, u16) {
        self.cleanup();
        (0, Error::Success as _)
    }

    fn reject(&mut self, _: &mut [u8]) -> (usize, u16) {
        self.cleanup();
        (0, Error::CommandNotAllowed as _)
    }
}
