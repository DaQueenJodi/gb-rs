use proc_macro_error::{proc_macro_error, abort_call_site};
use proc_macro::TokenStream;

use syn::parse::{ParseStream, Parse};
use syn::{LitStr, Ident};
use syn::parse_macro_input;
use syn::Token;

use quote::quote;


struct CmpArg {
    var: Ident,
    mask: u8,
    comp: u8
}

impl Parse for CmpArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let var = input.parse().expect("invalid variable ident");

        let _punct: Token![,] = input.parse().expect("comma required here");

        let byte: BitArgs = input.parse().expect("invalid mask/comp mix");

        let mask_bits: Vec<u32> = byte.bits.iter().map(|c| if let '1' | '0' = c { 1                      } else { 0 }).collect();
        let comp_bits: Vec<u32> = byte.bits.iter().map(|c| if let '1' | '0' = c { c.to_digit(2).unwrap() } else { 0 }).collect();

        let mask = mask_bits.iter().enumerate().fold(0, |acc, (i, b)| acc + (b << (7 - i)));
        let comp = comp_bits.iter().enumerate().fold(0, |acc, (i, b)| acc + (b << (7 - i)));


        let mask = mask.try_into().unwrap();
        let comp = comp.try_into().unwrap();

        let ret = Self {
            var,
            mask,
            comp
        };

        Ok(ret)
    }
}

struct CapArg {
    var: Ident,
    mask: u8,
    off: u8
}

impl Parse for CapArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let var = input.parse().expect("invalid variable ident");

        let _punct: Token![,] = input.parse().expect("comma required here");

        let byte: BitArgs = input.parse().expect("invalid mask/comp mix");

        let mask_bits: Vec<u32> = byte.bits.iter().map(|c| if let '1' | '0' = c { 0 } else { 1 }).collect();

        let mask = mask_bits.iter().enumerate().fold(0, |acc, (i, b)| acc + (b << (7 - i)));


        let mask: u8 = mask.try_into().unwrap();

        let off: u8 = mask.trailing_zeros().try_into().unwrap();

        let ret = Self {
            var,
            mask,
            off
        };

        Ok(ret)
    }
}

struct BitArgs {
    bits: [char; 8]
}

impl Parse for BitArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let str_lit: LitStr = input.parse().expect("bits has to be a string literal");
        let string = str_lit.value();

        let chars: Vec<char> = string.chars().collect();
        let len = chars.len();
        if  len != 8 {
            abort_call_site!("literal mask has to be exactly 8 characters, but it is {}", len);
        }
        let mut bits: [char; 8] = Default::default();
        for (i, bit) in chars.iter().enumerate() {
            bits[i] = *bit;
        }
        Ok(BitArgs {
            bits
        })
    }
}


#[proc_macro_error]
#[proc_macro]
pub fn bitcmp(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as CmpArg);
    let (var, mask, comp) = (input.var, input.mask, input.comp);
    quote!{
        (#var & #mask == #comp)
    }.into()
}

#[proc_macro_error]
#[proc_macro]
pub fn bitcap(stream: TokenStream) -> TokenStream {
    let CapArg { var, mask, off } = parse_macro_input!(stream as CapArg);
    quote!{
        ((#var & #mask) >> #off)
    }.into()
}
