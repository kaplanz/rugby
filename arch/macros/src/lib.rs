//! Derive macros for `rugby-arch`.

#![warn(clippy::pedantic)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Data, DeriveInput, Fields, Ident, LitInt, Token, parse_macro_input};

/// Derives an implementation of the `Memory` trait.
///
/// # Usage
///
/// The memory map is declared inline on the struct's fields using the
/// `#[mmap(...)]` helper attribute. Each attribute maps an address or an
/// inclusive range of addresses onto its field, with accesses delegated to the
/// field's own `Memory` implementation. Fields may declare any number of
/// attributes, each on its own line, and fields without any are left unmapped.
/// Declaration order is priority order: the first matching arm wins. Unmatched
/// addresses error with `Error::Range`.
///
/// Each attribute accepts the following options after the address:
///
/// - `mask = <lit>` delegates with the address masked (`addr & mask`), folding
///   the mapped range onto the field's own address space.
/// - `gate = <method>` only matches the arm while the predicate
///   `self.<field>.<method>()` holds.
///
/// ```ignore
/// #[derive(Memory)]
/// struct Board {
///     #[mmap(0x0000..=0x00ff, gate = ready)]
///     boot: Boot,
///     #[mmap(0xc000..=0xdfff, mask = 0x1fff)]
///     #[mmap(0xe000..=0xfdff, mask = 0x1fff)]
///     wram: Wram,
///     #[mmap(0xff46)]
///     dma: Dma,
/// }
/// ```
#[proc_macro_derive(Memory, attributes(mmap))]
pub fn memory(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Expands the `Memory` derive.
fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // Extract named fields
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "expected a struct"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &data.fields,
            "expected named fields",
        ));
    };

    // Generate match arms
    let mut read = Vec::new();
    let mut write = Vec::new();
    for field in &fields.named {
        let name = field.ident.as_ref().expect("field is named");
        for attr in &field.attrs {
            if !attr.path().is_ident("mmap") {
                continue;
            }
            let arm: Arm = attr.parse_args()?;
            let pat = &arm.addr;
            let addr = arm
                .mask
                .as_ref()
                .map_or_else(|| quote!(addr), |mask| quote!(addr & #mask));
            let cond = arm.gate.as_ref().map(|gate| quote!(if self.#name.#gate()));
            read.push(quote! {
                #pat #cond => {
                    ::rugby_arch::mem::Memory::read(&self.#name, #addr)
                }
            });
            write.push(quote! {
                #pat #cond => {
                    ::rugby_arch::mem::Memory::write(&mut self.#name, #addr, data)
                }
            });
        }
    }

    // Implement trait
    let name = &input.ident;
    let data = if write.is_empty() {
        quote!(_)
    } else {
        quote!(data)
    };
    Ok(quote! {
        #[automatically_derived]
        impl ::rugby_arch::mem::Memory for #name {
            fn read(&self, addr: u16) -> ::rugby_arch::mem::Result<u8> {
                match addr {
                    #(#read)*
                    _ => Err(::rugby_arch::mem::Error::Range),
                }
            }

            fn write(&mut self, addr: u16, #data: u8) -> ::rugby_arch::mem::Result<()> {
                match addr {
                    #(#write)*
                    _ => Err(::rugby_arch::mem::Error::Range),
                }
            }
        }
    })
}

/// A parsed `#[mmap(...)]` attribute.
struct Arm {
    /// Mapped address pattern.
    addr: proc_macro2::TokenStream,
    /// Address mask.
    mask: Option<LitInt>,
    /// Gate method.
    gate: Option<Ident>,
}

impl Parse for Arm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse address
        let base: LitInt = input.parse()?;
        let addr = if input.peek(Token![..=]) {
            let _: Token![..=] = input.parse()?;
            let last: LitInt = input.parse()?;
            quote!(#base..=#last)
        } else {
            quote!(#base)
        };
        // Parse options
        let mut mask = None;
        let mut gate = None;
        while !input.is_empty() {
            let _: Token![,] = input.parse()?;
            let key: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            match key.to_string().as_str() {
                "mask" => mask = Some(input.parse()?),
                "gate" => gate = Some(input.parse()?),
                _ => return Err(syn::Error::new(key.span(), "unknown option")),
            }
        }
        Ok(Self { addr, mask, gate })
    }
}
