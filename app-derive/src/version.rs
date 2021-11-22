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
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Error, Ident, LitStr};

use std::path::{Path, PathBuf};

struct Definition {
    name: String,
    value: u8,
}

pub fn version(input: TokenStream) -> TokenStream {
    let data_filepath = parse_macro_input!(input as LitStr);

    let data = match retrieve_data(data_filepath.value(), data_filepath.span()) {
        Err(e) => return e.into_compile_error().into(),
        Ok(data) => data,
    };

    let defs = data.into_iter().map(|Definition { name, value }| {
        let name = Ident::new(&name, Span::call_site());
        quote! {
            pub const #name : ::core::primitive::u8 = #value;
        }
    });

    let out = quote! {
        #(#defs )*
    };

    out.into()
}

fn retrieve_data(path: impl AsRef<Path>, path_span: Span) -> Result<Vec<Definition>, Error> {
    let base_path: PathBuf = ::std::env::var_os("CARGO_MANIFEST_DIR")
        .expect("Missing `CARGO_MANIFEST_DIR` env var")
        .into();

    let mut data_path = base_path;
    data_path.push(path.as_ref());

    let data_path = match data_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            return Err(Error::new(
                path_span,
                format!(
                    "Invalid path provided. Input path: {}; err={:?}",
                    data_path.display(),
                    err
                ),
            ));
        }
    };

    match std::fs::read_to_string(&data_path) {
        Ok(contents) => {
            contents
                .lines()
                .enumerate()
                .map(|(i, l)| (i, l.trim()))
                //ignore empty lines (maybe they were all whitespace so trim made them empty)
                .filter(|(_, l)| !l.is_empty())
                //ignore all lines that start with # as we consider them comments
                .filter(|(_, l)| !l.starts_with('#'))
                //find '=' and split the string at that in 2 pieces
                .map(|(i, l)| (i, l.splitn(2, '=').collect::<Vec<_>>()))
                .map(|(i, vec)| {
                    match vec.as_slice() {
                        &[name, value] => {
                            //parse value as u8
                            value
                                .parse::<u8>()
                                .map(|value| {
                                    //so we can construct our definition
                                    Definition {
                                        name: name.to_string(),
                                        value,
                                    }
                                })
                                .map_err(|parse| {
                                    //or report error for that value
                                    Error::new(
                                        path_span,
                                        format!(
                                            "Could not parse value #{:?} (@ #L{}) as u8: {:?}",
                                            name, i, parse
                                        ),
                                    )
                                })
                        }
                        &[_] => Err(Error::new(path_span, format!("Format invalid @ #L{}", i))),
                        &[] | &[_, _, ..] => unreachable!(),
                    }
                })
                .collect::<Result<Vec<_>, _>>()
        }
        Err(err) => Err(Error::new(
            path_span,
            format!("Could not read file. Path: {:?}; err={:?}", data_path, err),
        )),
    }
}
