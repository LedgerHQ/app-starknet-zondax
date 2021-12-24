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

use crate::{constants::STARK_SIGN_BUFFER_MIN_LENGTH, sys};
use sys::{crypto::bip32::BIP32Path, errors::Error};

#[derive(Clone, Copy)]
pub struct PublicKey(pub(crate) sys::crypto::stark::PublicKey);

impl PublicKey {
    pub const MAX_LEN: usize = 65;

    pub fn curve(&self) -> Curve {
        Curve::Stark256
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Curve {
    Stark256,
}

impl TryFrom<u8> for Curve {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Stark256),
            _ => Err(()),
        }
    }
}

impl From<Curve> for u8 {
    fn from(from: Curve) -> Self {
        match from {
            Curve::Stark256 => 0,
        }
    }
}

impl From<Curve> for sys::crypto::Curve {
    fn from(from: Curve) -> Self {
        match from {
            Curve::Stark256 => Self::Stark256,
        }
    }
}

impl TryFrom<sys::crypto::Curve> for Curve {
    type Error = ();

    fn try_from(ccrv: sys::crypto::Curve) -> Result<Self, Self::Error> {
        use sys::crypto::Curve as CCurve;

        match ccrv {
            CCurve::Stark256 => Ok(Self::Stark256),
            #[allow(unreachable_patterns)]
            //this isn't actually unreachable because CCurve mock is just incomplete
            _ => Err(()),
        }
    }
}

pub struct SecretKey<const B: usize>(sys::crypto::stark::SecretKey<B>);

pub enum SignError {
    BufferTooSmall,
    Sys(Error),
}

impl<const B: usize> SecretKey<B> {
    pub fn new(path: BIP32Path<B>) -> Self {
        Self(sys::crypto::stark::SecretKey::new(path))
    }

    pub fn into_public(self) -> Result<PublicKey, Error> {
        self.0.public().map(PublicKey)
    }

    #[inline(never)]
    pub fn into_public_into(self, out: &mut MaybeUninit<PublicKey>) -> Result<(), Error> {
        let inner_pk: &mut MaybeUninit<_> =
            //this is safe because the pointer is valid
            unsafe { &mut *addr_of_mut!((*out.as_mut_ptr()).0).cast() };

        self.0.public_into(inner_pk)
    }

    pub fn curve(&self) -> Curve {
        Curve::Stark256
    }

    pub fn sign(&self, data: &[u8], out: &mut [u8]) -> Result<usize, SignError> {
        if out.len() < STARK_SIGN_BUFFER_MIN_LENGTH {
            return Err(SignError::BufferTooSmall);
        }

        self.0
            .sign(data, out) //pass Sha256 for the signature nonce hasher
            .map_err(SignError::Sys)
    }
}

impl Curve {
    pub fn to_secret<const B: usize>(self, path: &BIP32Path<B>) -> SecretKey<B> {
        SecretKey::new(*path)
    }
}
