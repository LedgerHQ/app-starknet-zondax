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
use std::convert::{TryFrom, TryInto};

use crate::{constants::EDWARDS_SIGN_BUFFER_MIN_LENGTH, sys, utils::ApduPanic};
use sys::{crypto::bip32::BIP32Path, errors::Error, hash::Sha256};

#[derive(Clone, Copy)]
pub struct PublicKey(pub(crate) sys::crypto::ecfp256::PublicKey);

impl PublicKey {
    pub fn compress(&mut self) -> Result<(), Error> {
        self.0.compress()
    }

    pub fn curve(&self) -> Curve {
        //this unwrap is ok because the curve
        // can only be initialized by the library and not the user

        self.0.curve().try_into().apdu_unwrap()
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
    Ed25519,
}

impl TryFrom<u8> for Curve {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ed25519),
            _ => Err(()),
        }
    }
}

impl From<Curve> for u8 {
    fn from(from: Curve) -> Self {
        match from {
            Curve::Ed25519 => 0,
        }
    }
}

impl From<Curve> for sys::crypto::Curve {
    fn from(from: Curve) -> Self {
        match from {
            Curve::Ed25519 => Self::Ed25519,
        }
    }
}

impl TryFrom<sys::crypto::Curve> for Curve {
    type Error = ();

    fn try_from(ccrv: sys::crypto::Curve) -> Result<Self, Self::Error> {
        use sys::crypto::Curve as CCurve;

        match ccrv {
            CCurve::Ed25519 => Ok(Self::Ed25519),
            #[allow(unreachable_patterns)]
            //this isn't actually unreachable because CCurve mock is just incomplete
            _ => Err(()),
        }
    }
}

pub struct SecretKey<const B: usize>(sys::crypto::ecfp256::SecretKey<B>);

pub enum SignError {
    BufferTooSmall,
    Sys(Error),
}

impl<const B: usize> SecretKey<B> {
    pub fn new(curve: Curve, path: BIP32Path<B>) -> Self {
        use sys::crypto::Mode;

        Self(sys::crypto::ecfp256::SecretKey::new(
            Mode::BIP32,
            curve.into(),
            path,
        ))
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
        //this unwrap is ok because the curve
        // can only be initialized by the library and not the user

        self.0.curve().try_into().apdu_unwrap()
    }

    pub fn sign(&self, data: &[u8], out: &mut [u8]) -> Result<usize, SignError> {
        match self.curve() {
            Curve::Ed25519 if out.len() < EDWARDS_SIGN_BUFFER_MIN_LENGTH => {
                Err(SignError::BufferTooSmall)
            }

            Curve::Ed25519 => self
                .0
                .sign::<Sha256>(data, out) //pass Sha256 for the signature nonce hasher
                .map_err(SignError::Sys),
        }
    }
}

impl Curve {
    pub fn to_secret<const B: usize>(self, path: &BIP32Path<B>) -> SecretKey<B> {
        SecretKey::new(self, *path)
    }
}
