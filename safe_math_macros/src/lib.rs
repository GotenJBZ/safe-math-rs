use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::quote;
use syn::{BinOp, Expr, ExprBinary, Ident, ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn safe_math(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let orig_block = *input_fn.block;
    let new_block = rewrite_block(orig_block);
    input_fn.block = Box::new(new_block);
    TokenStream::from(quote! { #input_fn })
}

fn rewrite_block(block: syn::Block) -> syn::Block {
    use syn::fold::{self, Fold};
    struct MathRewriter;

    impl Fold for MathRewriter {
        fn fold_expr(&mut self, expr: Expr) -> Expr {
            let safe_math_crate = {
                match crate_name("safe_math").expect("safe_math crate not found") {
                    FoundCrate::Itself => Ident::new("::safe_math", Span::call_site()),
                    FoundCrate::Name(crate_name) => Ident::new(&crate_name, Span::call_site()),
                }
            };

            match expr {
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Add(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #safe_math_crate::safe_add(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Sub(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #safe_math_crate::safe_sub(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Mul(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #safe_math_crate::safe_mul(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Div(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #safe_math_crate::safe_div(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::Rem(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #safe_math_crate::safe_rem(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::AddAssign(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #left = #safe_math_crate::safe_add(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::SubAssign(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #left = #safe_math_crate::safe_sub(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::MulAssign(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #left = #safe_math_crate::safe_mul(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::DivAssign(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #left = #safe_math_crate::safe_div(#left, #right)? }
                }
                Expr::Binary(ExprBinary {
                    left,
                    op: BinOp::RemAssign(_),
                    right,
                    ..
                }) => {
                    let left = self.fold_expr(*left);
                    let right = self.fold_expr(*right);
                    syn::parse_quote! { #left = #safe_math_crate::safe_rem(#left, #right)? }
                }
                _ => fold::fold_expr(self, expr),
            }
        }
    }
    MathRewriter.fold_block(block)
}
