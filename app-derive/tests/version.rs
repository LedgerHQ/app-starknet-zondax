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

//! Test the version macro

use template_app_derive::version;

version!("../app/Makefile.version");

#[test]
fn version() {
    let _major = APPVERSION_M;
    let _minor = APPVERSION_N;
    let _patch = APPVERSION_P;
}
