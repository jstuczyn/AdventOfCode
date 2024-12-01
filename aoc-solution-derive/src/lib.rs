// Copyright 2023 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::aoc::AocContainer;
use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::ToTokens;
use syn::DeriveInput;

mod aoc;

#[proc_macro_derive(Aoc, attributes(aoc))]
pub fn derive_aoc_solution(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, ident, .. } = syn::parse_macro_input!(input);

    let aoc_attributes = aoc::parse_aoc_attrs(&attrs).unwrap_or_default();

    AocContainer::new(aoc_attributes, ident)
        .to_token_stream()
        .into()
}

trait ResultExt<T> {
    fn unwrap_or_abort(self) -> T;

    #[allow(dead_code)]
    fn expect_or_abort(self, message: &str) -> T;
}

impl<T> ResultExt<T> for Result<T, syn::Error> {
    fn unwrap_or_abort(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => abort!(error.span(), format!("{error}")),
        }
    }

    fn expect_or_abort(self, message: &str) -> T {
        match self {
            Ok(value) => value,
            Err(error) => abort!(error.span(), format!("{error}: {message}")),
        }
    }
}
