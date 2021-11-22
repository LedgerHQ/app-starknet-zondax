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
use core::{mem::MaybeUninit, ptr::addr_of_mut};
use std::convert::TryFrom;

use zemu_sys::{Show, ViewError, Viewable};

use crate::{
    constants::ApduError as Error,
    crypto,
    dispatcher::ApduHandler,
    handlers::handle_ui_message,
    sys::{
        self,
        hash::{Hasher, Sha256},
        Error as SysError,
    },
    utils::{hex_encode, ApduBufferRead, ApduPanic},
};

pub struct GetPublicKey;

impl GetPublicKey {
    /// Retrieve the public key with the given curve and bip32 path
    #[inline(never)]
    pub fn new_key_into<const B: usize>(
        curve: crypto::Curve,
        path: &sys::crypto::bip32::BIP32Path<B>,
        out: &mut MaybeUninit<crypto::PublicKey>,
    ) -> Result<(), SysError> {
        sys::zemu_log_stack("GetAddres::new_key\x00");
        curve.to_secret(path).into_public_into(out)?;

        //this is safe because it's initialized
        // also unwrapping is fine because the ptr is valid
        let pkey = unsafe { out.as_mut_ptr().as_mut().apdu_unwrap() };
        pkey.compress()
    }
}

impl ApduHandler for GetPublicKey {
    #[inline(never)]
    fn handle<'apdu>(
        flags: &mut u32,
        tx: &mut u32,
        buffer: ApduBufferRead<'apdu>,
    ) -> Result<(), Error> {
        sys::zemu_log_stack("GetPublicKey::handle\x00");

        *tx = 0;

        let req_confirmation = buffer.p1() >= 1;
        let curve = crypto::Curve::try_from(buffer.p2()).map_err(|_| Error::InvalidP1P2)?;

        let cdata = buffer.payload().map_err(|_| Error::DataInvalid)?;
        let bip32_path =
            sys::crypto::bip32::BIP32Path::<6>::read(cdata).map_err(|_| Error::DataInvalid)?;

        let mut ui = MaybeUninit::<AddrUI>::uninit();

        //initialize public key
        {
            //get ui *mut
            let ui = ui.as_mut_ptr();
            //get `pkey` *mut,
            // cast to MaybeUninit *mut
            //SAFE: `as_mut` it to &mut MaybeUninit (safe because it's MaybeUninit)
            // unwrap the option as it's guarantee valid pointer
            let key =
                unsafe { addr_of_mut!((*ui).pkey).cast::<MaybeUninit<_>>().as_mut() }.apdu_unwrap();
            Self::new_key_into(curve, &bip32_path, key).map_err(|_| Error::ExecutionError)?;
        }

        //safe because it's all initialized now
        // even tho the hash isn't, we aren't gonna read from it
        let mut ui = unsafe { ui.assume_init() };

        //actually compute pkey hash
        Sha256::digest_into(ui.pkey.as_ref(), &mut ui.hash).map_err(|_| Error::ExecutionError)?;

        if req_confirmation {
            unsafe { ui.show(flags) }.map_err(|_| Error::ExecutionError)
        } else {
            //we don't need to show so we execute the "accept" already
            // this way the "formatting" to `buffer` is all in the ui code
            let (sz, code) = ui.accept(buffer.write());

            if code != Error::Success as u16 {
                Err(Error::try_from(code).map_err(|_| Error::ExecutionError)?)
            } else {
                *tx = sz as u32;
                Ok(())
            }
        }
    }
}

pub struct AddrUI {
    pub pkey: crypto::PublicKey,
    hash: [u8; 32],
}

impl Viewable for AddrUI {
    fn num_items(&mut self) -> Result<u8, ViewError> {
        Ok(1)
    }

    fn render_item(
        &mut self,
        item_n: u8,
        title: &mut [u8],
        message: &mut [u8],
        page: u8,
    ) -> Result<u8, ViewError> {
        use bolos::{pic_str, PIC};

        if let 0 = item_n {
            let title_content = pic_str!(b"Public Key");
            title[..title_content.len()].copy_from_slice(title_content);

            let mut mex = [0; 64];
            let len = hex_encode(&self.hash[..], &mut mex).apdu_unwrap();

            handle_ui_message(&mex[..len], message, page)
        } else {
            Err(ViewError::NoData)
        }
    }

    fn accept(&mut self, out: &mut [u8]) -> (usize, u16) {
        let pkey = self.pkey.as_ref();
        let mut tx = 0;

        out[tx] = pkey.len() as u8;
        tx += 1;
        out[tx..tx + pkey.len()].copy_from_slice(pkey);
        tx += pkey.len();

        out[tx..tx + self.hash.len()].copy_from_slice(&self.hash[..]);
        tx += self.hash.len();

        (tx, Error::Success as _)
    }

    fn reject(&mut self, _: &mut [u8]) -> (usize, u16) {
        (0, Error::CommandNotAllowed as _)
    }
}
