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
use zemu_sys::ViewError;

use crate::{crypto::Curve, handlers::parser_common::ParserError};

///This trait defines the interface useful in the UI context
/// so that all the different OperationTypes or other items can handle their own UI
pub trait DisplayableItem {
    /// Returns the number of items to display
    fn num_items(&self) -> usize;

    /// This is invoked when a given page is to be displayed
    ///
    /// `item_n` is the item of the operation to display;
    /// guarantee: 0 <= item_n < self.num_items()
    /// `title` is the title of the item
    /// `message` is the contents of the item
    /// `page` is what page we are supposed to display, this is used to split big messages
    ///
    /// returns the total number of pages on success
    ///
    /// It's a good idea to always put `#[inline(never)]` on top of this
    /// function's implementation
    //#[inline(never)]
    fn render_item(
        &self,
        item_n: u8,
        title: &mut [u8],
        message: &mut [u8],
        page: u8,
    ) -> Result<u8, ViewError>;
}
