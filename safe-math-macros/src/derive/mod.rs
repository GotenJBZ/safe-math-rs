use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
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

macro_rules! gen_impl {
    ( $checked_ops:expr, $( ($var:ident, $op:ident, $trait:ident) ),* $(,)? ) => {
        $(
            let $var = if $checked_ops.contains(stringify!($op).trim_start_matches("safe_")) {
                quote! { <Self as ::safe_math::$trait>::$op(self, rhs) }
            } else {
                quote! { Err(::safe_math::SafeMathError::NotImplemented) }
            };
        )*
    };
}

// Macro to generate extra_impls TokenStream2 based on checked operations
macro_rules! gen_extra_impls {
    ( $checked_ops:expr, $name_ident:ident, $( ($op_lit:literal, $trait:ident, $checked_method:ident, $use_or_else:expr, $err_expr:expr) ),* $(,)? ) => {{
        let mut impls = TokenStream2::new();
        $(
            if $checked_ops.contains($op_lit) {
                let fn_ident = format_ident!("safe_{}", $op_lit);
                let trait_ident = syn::Ident::new(stringify!($trait), proc_macro2::Span::call_site());
                let method_ident = syn::Ident::new(stringify!($checked_method), proc_macro2::Span::call_site());
                if $use_or_else {
                    impls.extend(quote! {
                        #[diagnostic::do_not_recommend]
                        impl ::safe_math::#trait_ident for #$name_ident {
                            #[inline(always)]
                            fn #fn_ident(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                                self.#method_ident(&rhs).ok_or_else(|| { $err_expr })
                            }
                        }
                    });
                } else {
                    impls.extend(quote! {
                        #[diagnostic::do_not_recommend]
                        impl ::safe_math::#trait_ident for #$name_ident {
                            #[inline(always)]
                            fn #fn_ident(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                                self.#method_ident(&rhs).ok_or({ $err_expr })
                            }
                        }
                    });
                }
            }
        )*
        impls
    }};
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

    let name = &input.ident;

    gen_impl!(
        checked_ops,
        (add_impl, safe_add, SafeAdd),
        (sub_impl, safe_sub, SafeSub),
        (mul_impl, safe_mul, SafeMul),
        (div_impl, safe_div, SafeDiv),
        (rem_impl, safe_rem, SafeRem),
    );
    // Use macro to generate extra_impls
    let extra_impls = gen_extra_impls!(
        checked_ops,
        name,
        (
            "add",
            SafeAdd,
            checked_add,
            false,
            ::safe_math::SafeMathError::Overflow
        ),
        (
            "sub",
            SafeSub,
            checked_sub,
            false,
            ::safe_math::SafeMathError::Overflow
        ),
        (
            "mul",
            SafeMul,
            checked_mul,
            false,
            ::safe_math::SafeMathError::Overflow
        ),
        ("div", SafeDiv, checked_div, true, {
            if rhs == Self::default() {
                ::safe_math::SafeMathError::DivisionByZero
            } else {
                ::safe_math::SafeMathError::Overflow
            }
        }),
        (
            "rem",
            SafeRem,
            checked_rem,
            false,
            ::safe_math::SafeMathError::DivisionByZero
        ),
    );

    Ok(quote! {
        #[diagnostic::do_not_recommend]
        impl ::safe_math::SafeMathOps for #name {
            #[inline(always)]
            fn safe_add(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                #add_impl
            }

            #[inline(always)]
            fn safe_sub(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                #sub_impl
            }

            #[inline(always)]
            fn safe_mul(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                #mul_impl
            }

            #[inline(always)]
            fn safe_div(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                #div_impl
            }

            #[inline(always)]
            fn safe_rem(self, rhs: Self) -> Result<Self, ::safe_math::SafeMathError> {
                #rem_impl
            }
        }
        #extra_impls
    })
}
