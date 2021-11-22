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
pub mod public_key;
pub mod signing;
pub mod version;

#[cfg(feature = "dev")]
pub mod dev;

mod utils;
pub use utils::*;

pub mod resources {
    use super::lock::Lock;
    use bolos::{lazy_static, new_swapping_buffer, SwappingBuffer};

    #[lazy_static]
    pub static mut BUFFER: Lock<SwappingBuffer<'static, 'static, 0xFF, 0x1FFF>, BUFFERAccessors> =
        Lock::new(new_swapping_buffer!(0xFF, 0x1FFF));

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum BUFFERAccessors {
        Sign,
        #[cfg(feature = "dev")]
        Debug,
    }

    impl From<super::signing::Sign> for BUFFERAccessors {
        fn from(_: super::signing::Sign) -> Self {
            Self::Sign
        }
    }

    #[cfg(feature = "dev")]
    impl From<super::dev::Debug> for BUFFERAccessors {
        fn from(_: super::dev::Debug) -> Self {
            Self::Debug
        }
    }
}

pub mod lock;
