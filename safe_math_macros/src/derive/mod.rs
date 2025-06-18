use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_macro_input, DeriveInput, Meta};

const SAFE_MATH_OPS_ATTRIBUTE_NAME: &str = "SafeMathOps";
/// List of operations that can be specified inside the `#[SafeMathOps(...)]` attribute.
/// Keep this in sync with the match arms below.
const ALLOWED_OPS: &[&str] = &["add", "sub", "mul", "div", "rem"];

pub(crate) fn derive_safe_math_ops(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    expand_derive_safe_math_ops(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

macro_rules! generate_op_impls {
    ( $checked_ops:expr; $( ($op:ident, $impl_name:ident) ),* $(,)? ) => {
        $(
            let checked_op_fn = syn::Ident::new(&format!("checked_{}", stringify!($op)), proc_macro2::Span::call_site());
            let $impl_name = if $checked_ops.contains(stringify!($op)) {
                quote! { self.#checked_op_fn(&rhs).ok_or(SafeMathError::Overflow) }
            } else {
                quote! { Err(SafeMathError::NotImplemented) }
            };
        )*
    };
}

fn expand_derive_safe_math_ops(input: DeriveInput) -> syn::Result<TokenStream2> {
    let mut checked_ops: HashSet<String> = HashSet::new();

    for attr in &input.attrs {
        if attr.path().is_ident(SAFE_MATH_OPS_ATTRIBUTE_NAME) {
            match &attr.meta {
                // Expect the form `#[SafeMathOps(add, sub, ...)]`
                Meta::List(_) => {
                    // Parse the comma-separated list of paths inside the attribute.
                    let parsed_args = attr.parse_args_with(
                        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                    )?;

                    for arg in parsed_args {
                        if let Some(ident) = arg.get_ident() {
                            let ident_str = ident.to_string();
                            match ident_str.as_str() {
                                "add" | "sub" | "mul" | "div" | "rem" => {
                                    if !checked_ops.insert(ident_str.clone()) {
                                        return Err(syn::Error::new_spanned(
                                            arg,
                                            format!(
                                                "Duplicate operation '{}' in `#[SafeMathOps]` attribute. \
                                                 Each operation should be listed only once.",
                                                ident_str
                                            ),
                                        ));
                                    }
                                }
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        arg,
                                        format!(
                                            "Unknown operation '{}' in `#[SafeMathOps]` attribute. \
                                             Supported operations are: {}.",
                                            ident_str,
                                            ALLOWED_OPS.join(", ")
                                        ),
                                    ));
                                }
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                arg,
                                "Expected a simple identifier (e.g. `add`) inside `#[SafeMathOps]` attribute",
                            ));
                        }
                    }
                }
                // Handle cases like `#[SafeMathOps]` or other incorrect forms.
                _ => {
                    return Err(syn::Error::new_spanned(
                        &attr.meta,
                        "`#[SafeMathOps]` must be used with a list of operations, e.g. \
                         `#[SafeMathOps(add, sub)]`",
                    ));
                }
            }
        }
    }

    if checked_ops.is_empty() {
        return Err(syn::Error::new_spanned(
            &input,
            "`#[SafeMathOps]` requires at least one operation, e.g. `#[SafeMathOps(add, sub)]`",
        ));
    }

    generate_op_impls!(
        checked_ops;
        (add, add_impl),
        (sub, sub_impl),
        (mul, mul_impl),
        (div, div_impl),
        (rem, rem_impl),
    );

    let name = &input.ident;

    Ok(quote! {
        impl ::safe_math::SafeMathOps for #name {
            fn safe_add(self, rhs: Self) -> ::safe_math::SafeMathResult<Self> {
                #add_impl
            }

            fn safe_sub(self, rhs: Self) -> ::safe_math::SafeMathResult<Self> {
                #sub_impl
            }

            fn safe_mul(self, rhs: Self) -> ::safe_math::SafeMathResult<Self> {
                #mul_impl
            }

            fn safe_div(self, rhs: Self) -> ::safe_math::SafeMathResult<Self> {
                #div_impl
            }

            fn safe_rem(self, rhs: Self) -> ::safe_math::SafeMathResult<Self> {
                #rem_impl
            }
        }
    })
}
