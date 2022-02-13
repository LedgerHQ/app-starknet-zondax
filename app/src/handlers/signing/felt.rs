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
use bolos::{crypto::bip32::BIP32Path, pic_str, PIC};
use core::convert::TryFrom;
use zemu_sys::{Show, ViewError, Viewable};

use arrayref::array_mut_ref;

use crate::{
    constants::{ApduError as Error, BIP32_MAX_LENGTH},
    crypto::Curve,
    dispatcher::ApduHandler,
    handlers::{convert_der_to_rs, handle_ui_message, verify_bip32_path},
    sys,
    utils::{hex_encode, ApduBufferRead, ApduPanic, Uploader},
};

pub struct SignFelt;

impl SignFelt {
    pub const SIGN_ITEM_SIZE: usize = 32;

    //(actual_size, [u8; MAX_SIGNATURE_SIZE])
    #[inline(never)]
    pub fn sign<const LEN: usize>(
        path: &BIP32Path<LEN>,
        data: &[u8],
    ) -> Result<(bool, usize, [u8; 100]), Error> {
        let mut out = [0; 100];

        let (parity, sz) = Self::sign_into(path, data, &mut out)?;

        Ok((parity, sz, out))
    }

    #[inline(never)]
    pub fn sign_into<const LEN: usize>(
        path: &BIP32Path<LEN>,
        data: &[u8],
        out: &mut [u8; 100],
    ) -> Result<(bool, usize), Error> {
        let sk = Curve::Stark256.to_secret(path);

        let (parity, sz) = sk.sign(data, out).map_err(|_| Error::ExecutionError)?;

        Ok((parity, sz))
    }
}

impl ApduHandler for SignFelt {
    #[inline(never)]
    fn handle<'apdu>(
        flags: &mut u32,
        tx: &mut u32,
        buffer: ApduBufferRead<'apdu>,
    ) -> Result<(), Error> {
        sys::zemu_log_stack("Sign::handle\x00");

        *tx = 0;

        if let Some(upload) = Uploader::new(Self).upload(&buffer)? {
            let req_confirmation = upload.p2 >= 1;

            let path = BIP32Path::<BIP32_MAX_LENGTH>::read(upload.first)
                .map_err(|_| Error::DataInvalid)?;
            verify_bip32_path(&path)?;

            if upload.data.len() != 32 {
                return Err(Error::DataInvalid);
            }

            let unsigned_item = arrayref::array_ref![upload.data, 0, 32];

            let mut ui = SignFeltUI {
                path,
                felt: *unsigned_item,
            };

            if req_confirmation {
                unsafe { ui.show(flags) }.map_err(|_| Error::ExecutionError)
            } else {
                let (sz, code) = ui.accept(buffer.write());

                if code != Error::Success as u16 {
                    Err(Error::try_from(code).map_err(|_| Error::ExecutionError)?)
                } else {
                    *tx = sz as u32;
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }
}

pub(crate) struct SignFeltUI<const B: usize> {
    path: BIP32Path<B>,
    felt: [u8; SignFelt::SIGN_ITEM_SIZE],
}

impl<const B: usize> Viewable for SignFeltUI<B> {
    fn num_items(&mut self) -> Result<u8, ViewError> {
        Ok(1)
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
                let title_content = pic_str!(b"Sign Felt");
                title[..title_content.len()].copy_from_slice(title_content);

                let mut hex_buf = [0; SignFelt::SIGN_ITEM_SIZE * 2];
                //this is impossible that will error since the sizes are all checked
                let len = hex_encode(&self.felt[..], &mut hex_buf).apdu_unwrap();

                handle_ui_message(&hex_buf[..len], message, page)
            }
            _ => Err(ViewError::NoData),
        }
    }

    fn accept(&mut self, out: &mut [u8]) -> (usize, u16) {
        let (parity, sig_len, sig) = match SignFelt::sign(&self.path, &self.felt[..]) {
            Err(e) => return (0, e as _),
            Ok(k) => k,
        };

        let mut tx = 0;

        {
            //local copy of the mutable slice
            let out = &mut *out;

            //create 2 mut subslices (r, s) that are 32 byte long
            // and are referencing out
            let (r, out) = out.split_at_mut(32);
            let r = array_mut_ref![r, 0, 32];
            tx += 32;

            //no need to save the second part of the split
            let (s, _) = out.split_at_mut(32);
            let s = array_mut_ref![s, 0, 32];
            tx += 32;

            //write as R S V
            if convert_der_to_rs(&sig[..sig_len], r, s).is_err() {
                return (0, Error::ExecutionError as _);
            }
        }

        out[tx] = parity as u8;
        tx += 1;

        (tx, Error::Success as _)
    }

    fn reject(&mut self, _: &mut [u8]) -> (usize, u16) {
        (0, Error::CommandNotAllowed as _)
    }
}
