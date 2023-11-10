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

use crate::ResultExt;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::{Attribute, Error, Token};

pub struct AocContainer {
    attributes: AocAttr,
    ident: Ident,
}

impl AocContainer {
    pub fn new(attributes: AocAttr, ident: Ident) -> Self {
        AocContainer { attributes, ident }
    }
}

impl AocContainer {
    fn unimplemented_inner(&self, name: &str) -> TokenStream {
        // we're going to default to anyhow
        if self.attributes.error_ty.is_none() {
            quote! {
                let name = #name
                bail!("{name} hasn't been implemented")
            }
        } else {
            quote! {
                let name = #name
                panic!("{name} hasn't been implemented")
            }
        }
    }
}

impl ToTokens for AocContainer {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let AocContainer { attributes, ident } = self;
        let input_ty = if let Some(input_type) = &attributes.input_type {
            input_type.to_token_stream()
        } else {
            quote! {()}
        };

        let part1_ty = if let Some(part1_ty) = &attributes.part1_ty {
            part1_ty.to_token_stream()
        } else {
            quote! { String }
        };

        let part2_ty = if let Some(part2_ty) = &attributes.part2_ty {
            part2_ty.to_token_stream()
        } else {
            quote! { String }
        };

        // TODO: that technically requires anyhow import, but for the time being that's fine
        let error_ty = if let Some(error_ty) = &attributes.error_ty {
            error_ty.to_token_stream()
        } else {
            quote! { anyhow::Error }
        };

        // TODO: we need to ensure import of `AocInputParser`
        let parser_impl_inner = if let Some(parser) = &attributes.parser {
            quote! { #parser::parse_input(raw) }
        } else {
            self.unimplemented_inner("input parser")
        };

        let part1_impl_inner = if let Some(part1) = &attributes.part1 {
            if attributes.error_ty.is_none() {
                quote! { Ok(#part1(input)) }
            } else {
                quote! { #part1(input) }
            }
        } else {
            self.unimplemented_inner("part1")
        };

        let part2_impl_inner = if let Some(part2) = &attributes.part2 {
            if attributes.error_ty.is_none() {
                quote! { Ok(#part2(input)) }
            } else {
                quote! { #part2(input) }
            }
        } else {
            self.unimplemented_inner("part2")
        };

        tokens.extend(quote! {
            impl aoc_solution::AocSolution for #ident {
                type Input = #input_ty;
                type Error = #error_ty;
                type Part1Output = #part1_ty;
                type Part2Output = #part2_ty;

                fn parse_input(raw: &str) -> Result<Self::Input, Self::Error> {
                    #parser_impl_inner
                }

                fn part1(input: Self::Input) -> Result<Self::Part1Output, Self::Error> {
                    #part1_impl_inner
                }

                fn part2(input: Self::Input) -> Result<Self::Part2Output, Self::Error> {
                    #part2_impl_inner
                }
            }
        })
    }
}

impl Parse for AocAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE: &str =
            "unexpected attribute, expected any of: parser, part1, part2";

        let mut aocttr = AocAttr::default();

        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                Error::new(error.span(), format!("{EXPECTED_ATTRIBUTE}, {error}"))
            })?;
            let attribute = &*ident.to_string();

            match attribute {
                "input" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Type = input.parse()?;
                    aocttr.input_type = Some(ident);
                }
                "parser" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Ident = input.parse()?;
                    aocttr.parser = Some(ident);
                }
                "part1" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Ident = input.parse()?;
                    aocttr.part1 = Some(ident);
                }
                "part2" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Ident = input.parse()?;
                    aocttr.part2 = Some(ident);
                }
                "part1_output" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Type = input.parse()?;
                    aocttr.part1_ty = Some(ident);
                }
                "part2_output" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Type = input.parse()?;
                    aocttr.part2_ty = Some(ident);
                }
                "error" => {
                    input.parse::<Token![=]>()?;
                    let ident: syn::Type = input.parse()?;
                    aocttr.error_ty = Some(ident);
                }
                _ => {
                    return Err(Error::new(ident.span(), EXPECTED_ATTRIBUTE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(aocttr)
    }
}

#[derive(Default, Debug)]
pub struct AocAttr {
    // TODO: more concrete types?
    input_type: Option<syn::Type>,
    parser: Option<syn::Ident>,

    part1_ty: Option<syn::Type>,
    part2_ty: Option<syn::Type>,
    error_ty: Option<syn::Type>,

    part1: Option<syn::Ident>,
    part2: Option<syn::Ident>,
}

// struct AocPart {
//     output_ty: syn::Type,
//     exec: syn::Ident
// }

impl AocAttr {
    fn merge(mut self, other: AocAttr) -> Self {
        if other.input_type.is_some() {
            self.input_type = other.input_type
        }
        if other.parser.is_some() {
            self.parser = other.parser
        }
        if other.part1.is_some() {
            self.part1 = other.part1
        }
        if other.part2.is_some() {
            self.part2 = other.part2
        }
        if other.part1_ty.is_some() {
            self.part1_ty = other.part1_ty
        }
        if other.part2_ty.is_some() {
            self.part2_ty = other.part2_ty
        }
        if other.error_ty.is_some() {
            self.error_ty = other.error_ty
        }

        self
    }
}

pub fn parse_aoc_attrs(attrs: &[Attribute]) -> Option<AocAttr> {
    attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("aoc"))
        .map(|attribute| attribute.parse_args::<AocAttr>().unwrap_or_abort())
        .reduce(|acc, item| acc.merge(item))
}
