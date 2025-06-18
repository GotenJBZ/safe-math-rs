use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::atomic::{AtomicUsize, Ordering};
use syn::{parse_macro_input, spanned::Spanned, BinOp, Expr, ExprBinary, ItemFn};
#[cfg(feature = "derive")]
mod derive;

// Global counter for generating unique variable names
static TEMP_VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[proc_macro_attribute]
pub fn safe_math(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let orig_block = *input_fn.block;

    // ensure that the fn has a return type
    let return_type = match &input_fn.sig.output {
        syn::ReturnType::Type(_, ty) => ty,
        syn::ReturnType::Default => {
            return syn::Error::new(input_fn.sig.output.span(), "Function must return a Result")
                .to_compile_error()
                .into();
        }
    };

    // ensure that the return type is a Result
    let is_result = match &**return_type {
        syn::Type::Path(type_path) => {
            let segments = &type_path.path.segments;
            segments
                .last()
                .map(|seg| seg.ident == "Result")
                .unwrap_or(false)
        }
        _ => false,
    };

    if !is_result {
        return syn::Error::new(return_type.span(), "Function must return a Result")
            .to_compile_error()
            .into();
    }

    let new_block = rewrite_block(orig_block);
    input_fn.block = Box::new(new_block);
    TokenStream::from(quote! { #input_fn })
}

/// Generates a unique variable name that is extremely unlikely to collide
/// with user-defined variables
fn generate_unique_temp_var() -> syn::Ident {
    let counter = TEMP_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    // Use a very distinctive prefix that users are unlikely to use
    // Include the counter to ensure uniqueness across multiple macro invocations
    format_ident!(
        "__safe_math_temp_ref_{}_{}",
        std::process::id(), // Process ID for uniqueness across processes
        counter             // Counter for uniqueness within process
    )
}

fn rewrite_block(block: syn::Block) -> syn::Block {
    use syn::fold::{self, Fold};
    struct MathRewriter;
    impl Fold for MathRewriter {
        fn fold_expr(&mut self, expr: Expr) -> Expr {
            match expr {
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Add(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { ::safe_math::safe_add(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Sub(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { ::safe_math::safe_sub(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Mul(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { ::safe_math::safe_mul(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Div(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { ::safe_math::safe_div(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Rem(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { ::safe_math::safe_rem(#left, #right)? }
                }
                // Handle compound assignments by transforming them to regular assignments
                // to avoid double evaluation of the left-hand side
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::AddAssign(_),
                    right,
                    ..
                }) => {
                    let right = self.fold_expr(*right);
                    let temp_var = generate_unique_temp_var();
                    syn::parse_quote! {
                        {
                            let #temp_var = &mut #left;
                            *#temp_var = ::safe_math::safe_add(*#temp_var, #right)?;
                        }
                    }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::SubAssign(_),
                    right,
                    ..
                }) => {
                    let right = self.fold_expr(*right);
                    let temp_var = generate_unique_temp_var();
                    syn::parse_quote! {
                        {
                            let #temp_var = &mut #left;
                            *#temp_var = ::safe_math::safe_sub(*#temp_var, #right)?;
                        }
                    }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::MulAssign(_),
                    right,
                    ..
                }) => {
                    let right = self.fold_expr(*right);
                    let temp_var = generate_unique_temp_var();
                    syn::parse_quote! {
                        {
                            let #temp_var = &mut #left;
                            *#temp_var = ::safe_math::safe_mul(*#temp_var, #right)?;
                        }
                    }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::DivAssign(_),
                    right,
                    ..
                }) => {
                    let right = self.fold_expr(*right);
                    let temp_var = generate_unique_temp_var();
                    syn::parse_quote! {
                        {
                            let #temp_var = &mut #left;
                            *#temp_var = ::safe_math::safe_div(*#temp_var, #right)?;
                        }
                    }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::RemAssign(_),
                    right,
                    ..
                }) => {
                    let right = self.fold_expr(*right);
                    let temp_var = generate_unique_temp_var();
                    syn::parse_quote! {
                        {
                            let #temp_var = &mut #left;
                            *#temp_var = ::safe_math::safe_rem(*#temp_var, #right)?;
                        }
                    }
                }
                _ => fold::fold_expr(self, expr),
            }
        }
    }
    MathRewriter.fold_block(block)
}

#[cfg(feature = "derive")]
#[proc_macro_derive(SafeMathOps, attributes(SafeMathOps))]
pub fn derive_safe_math_ops(input: TokenStream) -> TokenStream {
    derive::derive_safe_math_ops(input)
}
