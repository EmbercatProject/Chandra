use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort;
use quote::{format_ident, quote};
use syn::fold::{self, Fold};
use syn::{parse_quote, BinOp, Expr,Pat, Stmt, Path, PathArguments};

use crate::parseatt::Structure;

pub struct ParseCPUfn {
    pub known_functions: HashMap<Path, Path>,
}

impl Fold for ParseCPUfn {
    fn fold_expr(&mut self, exp: Expr) -> Expr {
        match exp.clone() {
            Expr::Call(e) => {
                let func = *e.func;

                let f2 = func.clone();

                match f2 {
                    Expr::Path(mut p) => {
                        let attributes = {
                            let temp = p.clone();
                            let l = p.path.segments.last_mut().unwrap_or_else(|| abort!(temp, "Not a path"));
                            let t = l.arguments.clone();
                            l.arguments = PathArguments::None;
                            t
                        };

                        if let Some(v) = self.known_functions.clone().get(&p.path) {
                            let mut arguments: Vec<Expr> = e.args.into_iter().map(|a| self.fold_expr(a)).collect();

                            parse_quote!{
                                <#v #attributes>::cpu(#(#arguments,)*)
                            }
                        }

                        else {
                            let mut arguments: Vec<Expr> = e.args.into_iter().map(|a| self.fold_expr(a)).collect();

                            exp
                        }
                    }
                    _ => exp,
                }
            }
            _ => fold::fold_expr(self, exp),
        }
    }
}
