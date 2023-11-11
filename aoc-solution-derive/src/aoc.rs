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
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Attribute, Error, Token};

fn unimplemented_inner(custom_err: bool, name: &str) -> TokenStream {
    // we're going to default to anyhow
    if !custom_err {
        quote! { anyhow::bail!("{0} hasn't been implemented", #name) }
    } else {
        quote! { panic!("{0} hasn't been implemented", #name) }
    }
}

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
    fn wrap_runner(&self, runner: &Ident) -> TokenStream {
        if self.attributes.error_ty.is_none() {
            quote! { Ok(#runner(input)) }
        } else {
            quote! { #runner(input) }
        }
    }

    fn part1_impl(&self) -> TokenStream {
        if let Some(p1) = &self.attributes.part1 {
            if let Some(runner) = &p1.runner {
                return self.wrap_runner(runner);
            }
        }

        self.unimplemented_inner("part1")
    }

    fn part1_output(&self) -> TokenStream {
        if let Some(p1) = &self.attributes.part1 {
            if let Some(ty) = &p1.output_ty {
                return quote! { #ty };
            }
        }

        quote! { String }
    }

    fn part2_impl(&self) -> TokenStream {
        if let Some(p2) = &self.attributes.part2 {
            if let Some(runner) = &p2.runner {
                return self.wrap_runner(runner);
            }
        }

        self.unimplemented_inner("part2")
    }

    fn part2_output(&self) -> TokenStream {
        if let Some(p2) = &self.attributes.part2 {
            if let Some(ty) = &p2.output_ty {
                return quote! { #ty };
            }
        }

        quote! { String }
    }

    fn input_ty(&self) -> TokenStream {
        if let Some(input_type) = &self.attributes.input_type {
            input_type.to_token_stream()
        } else {
            quote! {()}
        }
    }

    // TODO: that technically requires anyhow import, but for the time being that's fine
    fn error_ty(&self) -> TokenStream {
        if let Some(error_ty) = &self.attributes.error_ty {
            error_ty.to_token_stream()
        } else {
            quote! { anyhow::Error }
        }
    }

    // TODO: we need to ensure import of `AocInputParser`
    fn parser_impl(&self) -> TokenStream {
        if let Some(parser) = &self.attributes.parser {
            quote! { #parser::parse_input(raw) }
        } else {
            self.unimplemented_inner("input parser")
        }
    }

    fn unimplemented_inner(&self, name: &str) -> TokenStream {
        unimplemented_inner(self.attributes.error_ty.is_some(), name)
    }
}

impl ToTokens for AocContainer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;

        let input_ty = self.input_ty();
        let error_ty = self.error_ty();
        let parser_impl = self.parser_impl();
        let p1_ty = self.part1_output();
        let p2_ty = self.part2_output();
        let p1_impl = self.part1_impl();
        let p2_impl = self.part2_impl();

        tokens.extend(quote! {
            impl aoc_solution::AocSolution for #ident {
                type Input = #input_ty;
                type Error = #error_ty;
                type Part1Output = #p1_ty;
                type Part2Output = #p2_ty;

                fn parse_input(raw: &str) -> Result<Self::Input, Self::Error> {
                    #parser_impl
                }

                fn part1(input: Self::Input) -> Result<Self::Part1Output, Self::Error> {
                    #p1_impl
                }

                fn part2(input: Self::Input) -> Result<Self::Part2Output, Self::Error> {
                    #p2_impl
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
                    let ident: AocPart = input.parse()?;
                    aocttr.part1 = Some(ident);
                }
                "part2" => {
                    input.parse::<Token![=]>()?;
                    let ident: AocPart = input.parse()?;
                    aocttr.part2 = Some(ident);
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
    error_ty: Option<syn::Type>,

    part1: Option<AocPart>,
    part2: Option<AocPart>,
}

#[derive(Debug, Clone, Default)]
struct AocPart {
    output_ty: Option<syn::Type>,
    runner: Option<syn::Ident>,
}

impl Parse for AocPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        const EXPECTED_ATTRIBUTE: &str = "unexpected attribute. expected one of: output, runner";

        let mut aoc_part = AocPart::default();

        while !content.is_empty() {
            let ident = content.parse::<Ident>().map_err(|error| {
                Error::new(error.span(), format!("{EXPECTED_ATTRIBUTE}, {error}"))
            })?;
            let attribute = &*ident.to_string();

            // every single attribute is in the form of `name = value`,
            // thus we should be able to parse out the Eq token
            content.parse::<Token![=]>()?;

            match attribute {
                "output" => {
                    let output_ty2: syn::Type = content.parse()?;
                    aoc_part.output_ty = Some(output_ty2);
                }
                "runner" => {
                    let runner2: Ident = content.parse()?;
                    aoc_part.runner = Some(runner2);
                }
                _ => return Err(syn::Error::new(ident.span(), EXPECTED_ATTRIBUTE)),
            }
            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        if aoc_part.runner.is_some() && aoc_part.output_ty.is_none() {
            return Err(Error::new(
                Span::call_site(),
                "could not use part runner without specifying return type",
            ));
        }

        Ok(aoc_part)
    }
}

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
