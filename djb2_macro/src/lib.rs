extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

const DJB2_MAGIC_KEY: u32 = 5381;

#[proc_macro]
pub fn define_djb2_hash_fn(input: TokenStream) -> TokenStream {
    let func_name = parse_macro_input!(input as syn::Ident);
    let hash: u32 = DJB2_MAGIC_KEY;
    let output = quote! {
        pub fn #func_name(data: &[u8]) -> u32 {
            let mut hash: u32 = #hash; // Start with a u32 hash
            for &byte in data {
                if byte == 0x0 {
                    continue;
                }
                hash = (hash << 5)
                .wrapping_add(hash)
                .wrapping_add(byte as u32); // hash * 33 + byte
            }
            hash
        }
    };

    TokenStream::from(output)
}

/// Computes a DJB2 hash at compile time and generates a constant.
#[proc_macro]
pub fn djb2_hash(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a string literal
    let input_str = parse_macro_input!(input as syn::LitByteStr);
    let inner = input_str.value();

    // Compute the hash at compile time
    let hash = djb2_hash_u8(&inner);

    // Generate the output tokens (a constant)
    let output = quote! {
        #hash
    };

    output.into()
}

fn djb2_hash_u8(data: &[u8]) -> u32 {
    let mut hash: u32 = DJB2_MAGIC_KEY; // Start with a u32 hash
    for &byte in data {
        if byte == 0x0 {
            continue;
        }
        hash = (hash << 5).wrapping_add(hash).wrapping_add(byte as u32); // hash * 33 + byte
    }
    hash
}
