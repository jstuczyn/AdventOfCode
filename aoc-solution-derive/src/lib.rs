use proc_macro::TokenStream;
// use quote::quote;
// use syn;

// TODO:
// allow something like:
// #[derive(Aoc)]
// #[aoc(parser = xxx)] -> infer Input type from parser
// #[aoc(part1 = xxx)] -> if not defined, return Result<(), Error>;
// #[aoc(part2 = xxx)] -> ibid
// struct DayXX;

#[proc_macro_derive(Aoc, attributes(aoc))]
pub fn derive_aoc_solution(input: TokenStream) -> TokenStream {
    input
}
