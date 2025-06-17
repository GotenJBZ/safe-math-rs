use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

const SAFE_MATH_OPS_ATTRIBUTE_NAME: &str = "SafeMathOps";

pub(crate) fn derive_safe_math_ops(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    expand_derive_safe_math_ops(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

macro_rules! generate_op_impl {
    ($impl_name:ident, $use_checked:ident) => {
        let op_name = stringify!($impl_name).replace("_impl", "");
        let checked_op_fn_name = format!("checked_{}", op_name);
        let checked_op_fn = syn::Ident::new(&checked_op_fn_name, proc_macro2::Span::call_site());
        let $impl_name = if $use_checked {
            quote! {
                self.#checked_op_fn(&rhs).ok_or(SafeMathError::Overflow)
            }
        } else {
            quote! {
                unimplemented!()
            }
        };
    };
}

fn expand_derive_safe_math_ops(input: DeriveInput) -> syn::Result<TokenStream2> {
    let mut use_checked_add = false;
    let mut use_checked_sub = false;
    let mut use_checked_mul = false;
    let mut use_checked_div = false;
    let mut use_checked_rem = false;

    for attr in &input.attrs {
        if attr.path().is_ident(SAFE_MATH_OPS_ATTRIBUTE_NAME) {
            if let Meta::List(_) = &attr.meta {
                let parsed_args = attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                )?;
                for arg in parsed_args {
                    if arg.is_ident("add") {
                        use_checked_add = true;
                    } else if arg.is_ident("sub") {
                        use_checked_sub = true;
                    } else if arg.is_ident("mul") {
                        use_checked_mul = true;
                    } else if arg.is_ident("div") {
                        use_checked_div = true;
                    } else if arg.is_ident("rem") {
                        use_checked_rem = true;
                    } else {
                        return Err(syn::Error::new_spanned(
                            arg,
                            "Unknown operation in SafeMathOps attribute",
                        ));
                    }
                }
            }
        }
    }
    // Extract the struct name from the input
    let name = &input.ident;

    // Only work with structs
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named.iter().collect::<Vec<_>>(),
            Fields::Unnamed(_) | Fields::Unit => {
                unimplemented!("SafeMathOps cannot be derived for unit structs");
            }
        },
        Data::Enum(_) | Data::Union(_) => {
            unimplemented!("SafeMathOps can only be derived for structs");
        }
    };

    //ensure all the fields have derived SafeMathOps, TODO check here
    for field in fields {
        let found_safe_math_ops_attributes: usize = field
            .attrs
            .iter()
            .filter(|attr| {
                let path = attr.path();
                let name = quote!(#path).to_string();
                name == SAFE_MATH_OPS_ATTRIBUTE_NAME
            })
            .count();
        if found_safe_math_ops_attributes > 1 {
            return Err(syn::Error::new(
                field.ident.as_ref().unwrap().span(),
                "Field must be derived SafeMathOps only once",
            ));
        }
    }

    generate_op_impl!(add_impl, use_checked_add);
    generate_op_impl!(sub_impl, use_checked_sub);
    generate_op_impl!(mul_impl, use_checked_mul);
    generate_op_impl!(div_impl, use_checked_div);
    generate_op_impl!(rem_impl, use_checked_rem);

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
