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
use crate::constants::ApduError;

pub struct Lock<T, A> {
    item: T,
    lock: Option<A>,
}

#[cfg_attr(test, derive(Debug))]
pub enum LockError {
    Busy,
    NotLocked,
    BadId,
}

impl<T, A> Lock<T, A> {
    pub const fn new(item: T) -> Self {
        Self { item, lock: None }
    }
}

impl<T, A: Eq> Lock<T, A> {
    ///Locks the resource (if available) and retrieve it
    pub fn lock(&mut self, acquirer: impl Into<A>) -> Result<&mut T, LockError> {
        let acq = acquirer.into();
        match self.lock {
            Some(ref a) if a == &acq => Ok(&mut self.item),
            //if it's busy we forcefully acquire the lock
            Some(_) | None => {
                self.lock = Some(acq);
                Ok(&mut self.item)
            }
        }
    }

    ///Acquire the resource if locked by `acquirer`
    pub fn acquire(&mut self, acquirer: impl Into<A>) -> Result<&mut T, LockError> {
        let acq = acquirer.into();
        match self.lock {
            Some(ref a) if a == &acq => Ok(&mut self.item),
            Some(_) => Err(LockError::Busy),
            None => Err(LockError::NotLocked),
        }
    }

    ///Release the resource if locker by `acquirer`
    pub fn release(&mut self, acquirer: impl Into<A>) -> Result<(), LockError> {
        let acq = acquirer.into();
        match self.lock {
            Some(ref a) if a == &acq => {
                self.lock = None;
                Ok(())
            }
            Some(_) => Err(LockError::BadId),
            None => Err(LockError::NotLocked),
        }
    }
}

impl From<LockError> for ApduError {
    fn from(lock: LockError) -> Self {
        match lock {
            LockError::NotLocked => Self::ExecutionError,
            LockError::Busy | LockError::BadId => Self::Busy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn build_lock(init: u32) -> Lock<u32, i32> {
        Lock::new(init)
    }

    #[test]
    fn nominal_use() {
        let mut lock = build_lock(0);

        *lock.lock(0).unwrap() += 1;

        assert_eq!(1, *lock.acquire(0).unwrap());

        lock.release(0).unwrap();
    }

    #[test]
    fn bad_accessors() {
        let mut lock = build_lock(32);
        lock.lock(32).unwrap();

        lock.acquire(0).unwrap_err();
        lock.release(0).unwrap_err();
    }

    #[test]
    fn lock_released() {
        let mut lock = build_lock(42);
        lock.lock(0).unwrap();
        lock.release(0).unwrap();

        lock.lock(1).unwrap();
    }

    #[test]
    fn force_lock() {
        let mut lock = build_lock(2);
        lock.lock(0).unwrap();
        lock.lock(1).unwrap();

        lock.acquire(0).unwrap_err();
        lock.acquire(1).unwrap();
    }
}
