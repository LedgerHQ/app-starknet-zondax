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
use arrayref::array_mut_ref;
use bolos::{
    crypto::bip32::BIP32Path,
    pic_str, PIC,
};
use zemu_sys::{Show, ViewError, Viewable};

use crate::{
    constants::{ApduError as Error, BIP32_MAX_LENGTH},
    crypto::Curve,
    dispatcher::ApduHandler,
    handlers::{convert_der_to_rs, handle_ui_message, verify_bip32_path},
    sys,
    utils::{hex_encode, ApduBufferRead, ApduPanic, Uploader},
};

pub struct ClearSign;

impl ClearSign {

    //(actual_size, [u8; MAX_SIGNATURE_SIZE])
    #[inline(never)]
    pub fn sign<const LEN: usize>(
        path: &BIP32Path<LEN>,
        data: &[u8],
    ) -> Result<(bool, usize, [u8; 100]), Error> {
        let sk = Curve::Stark256.to_secret(path);

        let mut out = [0; 100];
        let (parity, sz) = sk
            .sign(data, &mut out[..])
            .map_err(|_| Error::ExecutionError)?;

        Ok((parity, sz, out))
    }
}

impl ApduHandler for ClearSign {
    #[inline(never)]
    fn handle<'apdu>(
        flags: &mut u32,
        tx: &mut u32,
        buffer: ApduBufferRead<'apdu>,
    ) -> Result<(), Error> {
        sys::zemu_log_stack("ClearSign::handle\x00");

        *tx = 0;

        if let Some(upload) = Uploader::new(Self).upload(&buffer)? {

            let path = BIP32Path::<BIP32_MAX_LENGTH>::read(upload.dpath_data)
                .map_err(|_| Error::DataInvalid)?;
            
            verify_bip32_path(&path)?;

            let s = upload.data;

            let fields: (&[u8], &[u8]);
            fields = s.split_at(6 as usize);

            let (field1, field23) = fields.1.split_at(66);
            let (field2, field3) = field23.split_at(10);
           
            let mut ui = ClearSignUI {
                path,
                field0: fields.0,
                field1,
                field2,
                field3,
            };

            unsafe { ui.show(flags) }.map_err(|_| Error::ExecutionError)
        }
        else {
            Ok(()) 
        }
    }
}

pub(crate) struct ClearSignUI<const B: usize> {
    path: BIP32Path<B>,
    field0: &'static[u8],
    field1: &'static[u8],
    field2: &'static[u8],
    field3: &'static[u8],
}

impl<const B: usize> Viewable for ClearSignUI<B> {
    fn num_items(&mut self) -> Result<u8, ViewError> {
        Ok(4)
    }

    #[inline(never)]
    fn render_item(
        &mut self,
        item_n: u8,
        title: &mut [u8],
        message: &mut [u8],
        page: u8,
    ) -> Result<u8, ViewError> {
        match item_n {
            0 => {
                let title_content = pic_str!(b"ClearSign");
                title[..title_content.len()].copy_from_slice(title_content);

                handle_ui_message(&self.field0, message, page)
            }
            1 => {
                let title_content = pic_str!(b"ClearSign");
                title[..title_content.len()].copy_from_slice(title_content);

                handle_ui_message(&self.field1, message, page)
            }
            2 => {
                let title_content = pic_str!(b"ClearSign");
                title[..title_content.len()].copy_from_slice(title_content);

                handle_ui_message(&self.field2, message, page)
            }
            3 => {
                let title_content = pic_str!(b"ClearSign");
                title[..title_content.len()].copy_from_slice(title_content);

                handle_ui_message(&self.field3, message, page)
            }
            _ => Err(ViewError::NoData),
        }
    }

    fn accept(&mut self, out: &mut [u8]) -> (usize, u16) {
        sys::zemu_log_stack("Accept Clear Signing\x00");
        (65, Error::Success as _)
    }

    fn reject(&mut self, _: &mut [u8]) -> (usize, u16) {
        (0, Error::CommandNotAllowed as _)
    }
}
