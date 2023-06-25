use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort;
use quote::{format_ident, quote};
use syn::fold::{self, Fold};
use syn::{parse_quote, BinOp, Expr,Pat, Stmt, Path, PathArguments};

use crate::parseatt::Structure;

pub struct Parsefn {
    pub return_type: TokenStream,
    pub expr_type: TokenStream,
    pub var_levels: Vec<HashMap<Ident, TokenStream>>,
    pub vars: HashMap<Ident, TokenStream>,
    pub scope_depth: usize,
    pub known_structures: HashMap<Ident, Structure>,
    pub block_prev: TokenStream,
    pub block_prev_type: TokenStream,
    pub known_generics: HashMap<Ident, TokenStream>,
    pub crate_root: TokenStream,
    pub returns_struct: bool,
    pub known_functions: HashMap<Path, Path>,
    pub known_extensions: HashMap<Path, Path>,
}

impl Fold for Parsefn {
    fn fold_block(&mut self, i: syn::Block) -> syn::Block {
        let crate_root = self.crate_root.clone();
        self.scope_depth += 1;
        let scope = format_ident!("s_{}", self.scope_depth);
        self.var_levels.push(self.vars.clone());

        let prev = self.return_type.clone();
        self.return_type = quote!(#crate_root::core::operations::noop::Noop);

        let stmts: Vec<Stmt> = i.clone().stmts.into_iter().map(|s| self.fold_stmt(s)).collect();

        let old = self.block_prev.clone();
        let old_expr = self.block_prev_type.clone();

        let after = self.return_type.clone();
        let expr_return = self.expr_type.clone();

        self.return_type = quote!(#crate_root::core::operations::scope::Scope<#expr_return,#expr_return,#old_expr, #after,#old>);

        self.scope_depth -= 1;
        self.vars = self.var_levels.pop().unwrap_or_else(|| abort!(i, "Internal ERROR: Too less var levels"));

        parse_quote! {
            {
                let #scope = #crate_root::core::operations::scope::Scope::new::<#expr_return>();
                #(#stmts)*

                #scope
            }
        }
    }

    fn fold_expr(&mut self, e: Expr) -> Expr {
        let crate_root = self.crate_root.clone();

        match e {
            Expr::Assign(e) => {
                let op = e.eq_token;
                let left = *e.left;
                let right = self.fold_expr(*e.right);

                parse_quote! {
                    #crate_root::core::operations::set::set(&#left, #right)
                }
            }
            Expr::Binary(b) => {
                let mut left = self.fold_expr(*b.left.clone());

                let mut left_ty = self.return_type.clone();
                let l_expr_ty = self.expr_type.clone();

                let mut right = self.fold_expr(*b.right.clone());

                let mut right_ty = self.return_type.clone();
                let r_expr_ty = self.expr_type.clone();

                let expr_ty = match (l_expr_ty.is_empty(), r_expr_ty.is_empty()) {
                    (false, _) => l_expr_ty,
                    (_, false) => r_expr_ty,
                    (_, _) => abort!(b, "Can't infer Type, consider hinting literal Type"),
                };

                self.expr_type = expr_ty.clone();

                match (left_ty.is_empty(), right_ty.is_empty()) {
                    (false, false) => {}
                    (false, true) => { 
                        if let Expr::Lit(l) = right.clone() {
                            right_ty = expr_ty.clone(); 

                            if let syn::Lit::Float(_) = l.lit {
                                right = parse_quote!(<#right_ty>::from_float(#right));
                            } else {
                                right = parse_quote!(<#right_ty>::from_int(#right));
                            }
                            
                        } else {
                            abort!(right, "Can't infer type")
                        }
                    }
                    (true, false) => { 
                        if let Expr::Lit(l) = left.clone() {
                            left_ty = expr_ty.clone(); 
                            
                            if let syn::Lit::Float(_) = l.lit {
                                left = parse_quote!(<#left_ty>::from_float(#left));
                            } else {
                                left = parse_quote!(<#left_ty>::from_int(#left));
                            }
                            
                        } else {
                            abort!(right, "Can't infer type")
                        }
                     }
                    (true, true) => abort!(b, "Can't infer Type, consider hinting literal Type")
                }

                match &b.op {
                    BinOp::Add(a) => {
                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            #expr_ty, 
                            #crate_root::core::operations::add::Add<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);

                       // println!();
                       // println!("ADD: {}", self.return_type);
                       // println!();

                        parse_quote!(#crate_root::core::operations::add::add(#left, #right))
                    }
                    BinOp::Sub(a) => {
                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            #expr_ty, 
                            #crate_root::core::operations::subtract::Subtract<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);

                        parse_quote!(#crate_root::core::operations::subtract::subtract(#left, #right))
                    }
                    BinOp::Mul(m) => {
                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            #expr_ty, 
                            #crate_root::core::operations::multiply::Multiply<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);

                        parse_quote!(#crate_root::core::operations::multiply::multiply(#left, #right))
                    }
                    BinOp::Div(a) => {
                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            #expr_ty, 
                            #crate_root::core::operations::divide::Divide<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);

                        parse_quote!(#crate_root::core::operations::divide::divide(#left, #right))
                    }
                    BinOp::Ge(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::greater_than_or_equal::GreaterThanOrEqual<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);

                        parse_quote!(#crate_root::core::operations::greater_than_or_equal::greater_than_or_equal(#left, #right))
                    }
                    BinOp::Gt(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::greater_than::GreaterThan<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::greater_than::greater_than(#left, #right))
                    }
                    BinOp::Le(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::less_than_or_equal::LessThanOrEqual<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::less_than_or_equal::less_than_or_equal(#left, #right))
                    }
                    BinOp::Lt(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::less_than::LessThan<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::less_than::less_than(#left, #right))
                    }
                    BinOp::Eq(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::equal::Equal<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::equal::equal(#left, #right))
                    }
                    BinOp::Ne(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::not_equal::NotEqual<
                                #expr_ty, 
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::not_equal::not_equal(#left, #right))
                    }
                    BinOp::And(g) => {
                        self.expr_type = quote!(bool);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::and::And<
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::and::and(#left, #right))
                    }
                    BinOp::Or(g) => {
                        self.expr_type = quote!(bool);
                        
                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            bool, 
                            #crate_root::core::operations::or::Or<
                                #left_ty, 
                                #right_ty
                            >
                        >);
                        
                        parse_quote!(#crate_root::core::operations::or::or(#left, #right))
                    }
                    _ => abort!(b, "Unsupported binary Expression"),
                }
            }
            Expr::MethodCall(e) => {
                let func = e.method;
                let func_path = Path::from(func.clone());

                if let Some(v) = self.known_extensions.clone().get(&func_path) {
                    let mut arg_types = Vec::new();
                    let mut arguments = Vec::new();

                    arguments.push(self.fold_expr(*e.receiver));
                    arg_types.push(self.return_type.clone());

                    for arg in e.args.clone().into_iter() {
                        arguments.push(self.fold_expr(arg));
                        arg_types.push(self.return_type.clone());
                    }

                    let result_type = quote!(<#v as #crate_root::core::type_traits::ChandraExtensionFn>::Result);

                    self.return_type = quote!{
                        #crate_root::core::operation::OperationWrapper<
                            #result_type,
                            #crate_root::core::operations::call::Call<
                                #result_type,
                                <#v as #crate_root::core::type_traits::ChandraExtensionFn>::Inputs,
                                (#(#arg_types,)*),
                                <#v as #crate_root::core::type_traits::ChandraExtensionFn>::FunctionScope,
                            >
                        >
                    };

                    parse_quote!{
                        #crate_root::core::operations::call::call((#(#arguments,)*), <#v>::build_fn())
                    }
                } else {
                    abort!(func, &format!("Methods need an imported ChandraExtension like: use {} as {};", func, func))
                }
            }
            Expr::Call(e) => {
                let func = *e.func;

                let f2 = func.clone();

                let ty = self.return_type.clone();

                match f2 {
                    Expr::Path(mut p) => {
                        let attributes = {
                            let temp = p.clone();
                            let l = p.path.segments.last_mut().unwrap_or_else(|| abort!(temp, "Not a path"));
                            let t = l.arguments.clone();
                            l.arguments = PathArguments::None;
                            t
                        };
                        
                        p.attrs = Vec::new();
                        if let Some(v) = self.known_extensions.clone().get(&p.path) {
                            let mut arg_types = Vec::new();
                            let mut arguments = Vec::new();

                            for arg in e.args.clone().into_iter() {
                                arguments.push(self.fold_expr(arg));
                                arg_types.push(self.return_type.clone());
                            }

                            let result_type = quote!(<#v #attributes as #crate_root::core::type_traits::ChandraExtensionFn>::Result);

                            self.return_type = quote!{
                                #crate_root::core::operation::OperationWrapper<
                                    #result_type,
                                    #crate_root::core::operations::call::Call<
                                        #result_type,
                                        <#v #attributes as #crate_root::core::type_traits::ChandraExtensionFn>::Inputs,
                                        (#(#arg_types,)*),
                                        <#v #attributes as #crate_root::core::type_traits::ChandraExtensionFn>::FunctionScope,
                                    >
                                >
                            };

                            parse_quote!{
                                #crate_root::core::operations::call::call((#(#arguments,)*), <#v #attributes>::build_fn())
                            }
                        }

                        else if let Some(v) = self.known_functions.clone().get(&p.path) {
                            let mut arg_types = Vec::new();
                            let mut arguments = Vec::new();

                            for arg in e.args.clone().into_iter() {
                                arguments.push(self.fold_expr(arg));
                                arg_types.push(self.return_type.clone());
                            }

                            let result_type = quote!(<#v #attributes as #crate_root::core::type_traits::ChandraFn>::Result);

                            self.return_type = quote!{
                                #crate_root::core::operation::OperationWrapper<
                                    #result_type,
                                    #crate_root::core::operations::call::Call<
                                        #result_type,
                                        <#v #attributes as #crate_root::core::type_traits::ChandraFn>::Inputs,
                                        (#(#arg_types,)*),
                                        <#v #attributes as #crate_root::core::type_traits::ChandraFn>::FunctionScope,
                                    >
                                >
                            };

                            parse_quote!{
                                #crate_root::core::operations::call::call((#(#arguments,)*), <#v #attributes>::build_fn())
                            }
                        }

                        else if let Some(i) = p.path.get_ident() {
                            abort!(i, "Can't do that for now, sorry");

                        } else {
                            let args: Vec<Expr> = e.args.into_iter().map(|x| self.fold_expr(x)).collect();

                            let mut it = p.path.segments.into_iter();

                            let generic = it.next();
                            let function = it.next();
                            let none = it.next();

                            match (generic, function, none) {
                                (Some(g), Some(f), None) => {
                                    if let Some(known) = self.known_generics.get(&g.ident) {
                                        
                                        self.expr_type = known.clone();
                                        self.return_type = known.clone();

                                        parse_quote! {
                                            <#g>:: #f(#(#args,)*)
                                         }
                                    } else {
                                        abort!(g, "Needs to be a known Generic")
                                    }
                                }

                                _ => abort!(func, "Not a valid generic function")
                            }
                        }
                    }
                    _ => abort!(f2, "Not Path"),
                }
            }
            Expr::If(x) => {
                let prev = self.return_type.clone();

                let condition = self.fold_expr(*x.cond);
                let cond_ty = self.return_type.clone();

                let before = self.block_prev.clone();
                let before_expr = self.block_prev_type.clone();
                self.block_prev = quote!(#crate_root::core::operations::noop::Noop);
                self.block_prev_type = quote!(#crate_root::core::types::Void);

                let block = self.fold_block(x.then_branch.clone());
                let block_ty = self.return_type.clone();
                let block_expr = self.expr_type.clone();

                self.return_type = prev.clone();

                if let Some((_, els_raw)) = x.else_branch {
                    self.block_prev = quote!(#crate_root::core::operations::noop::Noop);
                    self.block_prev_type = quote!(#crate_root::core::types::Void);

                    let els = self.fold_expr(*els_raw);
                    let els_ty = self.return_type.clone();
                    let els_expr = self.expr_type.clone();
                    
                    self.block_prev = before;

                    self.return_type = quote!(
                        #crate_root::core::operation::OperationWrapper<
                            #block_expr,
                            #crate_root::core::operations::if_else::IfElse<
                                #block_expr, 
                                #cond_ty, 
                                <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::A,
                                <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::B,
                                <#els_ty as #crate_root::core::operations::scope::ScopeTrait>::A,
                                <#els_ty as #crate_root::core::operations::scope::ScopeTrait>::B,
                            >
                        >
                    );

                    parse_quote!{
                        #crate_root::core::operations::if_else::if_then_or_else(#condition, #block, #els)
                    }   
                } else {
                    self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                        #crate_root::core::types::Void, 
                        #before_expr, 
                        #prev, 
                        #before
                    >);

                    self.return_type = quote!(
                        #crate_root::core::operation::OperationWrapper<
                            #block_expr,
                            #crate_root::core::operations::if_else::IfElse<
                                #block_expr, 
                                #cond_ty, 
                                <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::A,
                                <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::B,
                                #crate_root::core::operations::noop::Noop,
                                #crate_root::core::operations::noop::Noop,
                            >
                        >
                    );

                    parse_quote!{
                        #crate_root::core::operations::if_else::if_then(#condition, #block)
                    }   
                } 
            }
            Expr::Index(ind) => {
                if let Expr::Path(p) = *ind.expr {
                    let i = p
                        .path
                        .get_ident()
                        .unwrap_or_else(|| abort!(p, "Needs to be a local variable 1"));

                    let ty = self.vars
                        .get(i)
                        .unwrap_or_else(|| abort!(p, "Needs to be a known variable"))
                        .clone();

                    let ind_expr = self.fold_expr(*ind.index);
                    
                    let ind_ty = self.return_type.clone();

                    self.return_type = parse_quote!(#crate_root::core::operation::OperationWrapper<<#ty as #crate_root::core::type_traits::IndexAble>::IndexResult, #crate_root::core::operations::index::Index<<#ty as #crate_root::core::type_traits::IndexAble>::IndexResult, #ty, #ind_ty>>);
                    self.expr_type = quote!(<#ty as #crate_root::core::type_traits::IndexAble>::IndexResult);

                    parse_quote!(#crate_root::core::operations::index::index(&#i, #ind_expr))
                } else {
                    abort!(ind, "Not a variable")
                }
            }
            Expr::Lit(l) => {
                match l.lit.suffix() {
                    "f32" => {
                        self.return_type = quote!(f32);
                        self.expr_type = quote!(f32);
                    }
                    "i32" => {
                        self.return_type = quote!(i32);
                        self.expr_type = quote!(i32);
                    }
                    "u32" => {
                        self.return_type = quote!(u32);
                        self.expr_type = quote!(u32);
                    }
                    "u64" => {
                        self.return_type = quote!(u64);
                        self.expr_type = quote!(u64);
                    }
                    _ => {
                        self.return_type = TokenStream::new();
                        self.expr_type = TokenStream::new();
                    },
                }

                fold::fold_expr(self, Expr::Lit(l))
            }
            Expr::Path(p) => {
                if let Some(i) = p.path.get_ident() {
                    //eprintln!("ExprPath");
                    let vTypeOption = self
                        .vars
                        .get(i)
                        .clone();

                    if let Some(vType) = vTypeOption {
                        self.expr_type = vType.clone();

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                            #vType, 
                            #crate_root::core::operations::get::Get<
                                #vType
                            >
                        >);

                        parse_quote!(#crate_root::core::operations::get::get(&#i))
                    } else if i.to_string() == "PhantomData" {
                        parse_quote!(PhantomData)
                    } else {
                        abort!(p, "Needs to be a known variable")
                    }

                } else {
                    abort!(p, "Currently only local values are supported")
                }
            }
            Expr::Field(f) => {
                match *f.clone().base {
                    Expr::Path(p) => {
                        if let Some(i) = p.path.get_ident() {
                            //eprintln!("ExprPath");

                            let member_ident = if let syn::Member::Named(named) = &f.member {
                                named
                            } else {
                                abort!(f.member, "Only named field structs are supported")
                            };

                            let vType = format_ident!("{}", self
                                .known_structures
                                .get(i)
                                .unwrap_or_else(|| abort!(p, "Needs to be a known structure"))
                                .get(&member_ident.to_string())
                                .unwrap_or_else(|| abort!(f.member, "Needs to be a known field"))
                                .clone());

                           // println!("");
                           // println!("{:?}", quote!(#vType));
                           // println!("");

                            self.expr_type = quote!(#vType);
                            self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                                #vType, 
                                #crate_root::core::operations::get::Get<
                                    #vType
                                >
                            >);

                            parse_quote!(#crate_root::core::operations::get::get(&#f))
                        } else {
                            abort!(p, "Currently only local values are supported")
                        }
                    }
                    _ => abort!(f, "Currently only local values are supported")
                }
            }
            Expr::Struct(s) => {
                let path = s.path.clone();

                let mut stream = TokenStream::new();

                for field in s.fields {
                    let assignment = self.fold_expr(field.expr);
                    let member = field.member;

                    stream.extend(quote!(#member: #assignment,))
                }

                self.returns_struct = true;

                self.return_type = quote!(
                    #path
                );

                parse_quote!(#path {#stream} )
            }
            Expr::Range(range) => {
                match (range.clone().start, range.clone().end) {
                    (Some(x), Some(y)) => {
                        let left = self.fold_expr(*x);
                        let left_ty = self.return_type.clone();

                        let right = self.fold_expr(*y);
                        let right_ty = self.return_type.clone();
                        let expr_typ = self.expr_type.clone();

                        self.return_type = quote!(
                            #crate_root::core::operation::OperationWrapper<#expr_typ,
                                #crate_root::core::operations::range::Range<#left_ty, #right_ty>
                            >);

                        parse_quote!(#crate_root::core::operations::range::range(#left, #right))
                    }

                    _ => abort!(range, "Ranges have to have both Limits")
                }
            }

            Expr::Cast(c) => {
                let ty = c.ty;
                self.expr_type = quote!(#ty);
                let res = self.fold_expr(*c.expr);
                self.expr_type = quote!(#ty);
                res
            }
            
            Expr::Reference(r) => {
                //eprintln!("ExprRef");
                parse_quote!((*#r).clone())
            }
            _ => fold::fold_expr(self, e),
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        let crate_root = self.crate_root.clone();
        
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() {
                    let prev = self.return_type.clone();
                    let prev_expr = self.expr_type.clone();

                    let init = self.fold_expr(*s.init.unwrap().expr);

                    let after = self.return_type.clone();

                    let expr_type = self.expr_type.clone();

                    self.return_type = quote!(
                        #crate_root::core::operation::OperationWrapper<
                            #crate_root::core::types::Void, 
                            #crate_root::core::operations::assign::Assign<
                                #expr_type, 
                                #after
                            >
                        >);
                    let old = self.block_prev.clone();
                    let old_expr = self.block_prev_type.clone();

                    self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                        #crate_root::core::types::Void, 
                        #old_expr, 
                        #prev, 
                        #old
                    >);

                    let pat = s.pat.clone();
                    let scope = format_ident!("s_{}", self.scope_depth);

                    if let Pat::Ident(p) = &pat {
                        let pat_name = format!("{}", p.ident);
                        self.vars.insert(p.ident.clone(), quote!(#expr_type));

                        //eprintln!("Local");

                        self.expr_type = quote!(#crate_root::core::types::Void);

                        parse_quote! {
                            let (#pat, #scope) = (
                                #crate_root::core::operations::assign::assign(::std::string::String::from(#pat_name), #init).0,
                                #scope.include(#crate_root::core::operations::assign::assign(::std::string::String::from(#pat_name), #init).1)
                            );
                        }
                    } else {
                        abort!(init, "abort")
                    }
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }

            Stmt::Expr(e, semi) => {
                let scope = format_ident!("s_{}", self.scope_depth);

                let exp: Expr = match e.clone() {
                    Expr::Assign(a) => {
                        let op = a.eq_token;
                        let mut left = *a.left;

                        let prev = self.return_type.clone();

                        let right = self.fold_expr(*a.right);
                        let after = self.return_type.clone();

                        let mut expr_ty = TokenStream::new();

                        let ret = match left.clone() {
                            Expr::Path(p) => {
                                let i = p
                                    .path
                                    .get_ident()
                                    .unwrap_or_else(|| abort!(e, "Needs to be a local variable 1"));
                                let ty = self.vars
                                    .get(i)
                                    .unwrap_or_else(|| abort!(e, "Needs to be a known variable"))
                                    .clone();

                                expr_ty = ty.clone();

                                quote!(#ty, #crate_root::core::operations::var::Variable<#ty>)
                            }
                            Expr::Index(ind) => {
                                if let Expr::Path(p) = *ind.expr {
                                    let i = p
                                        .path
                                        .get_ident()
                                        .unwrap_or_else(|| abort!(e, "Needs to be a local variable 1"));
                                    let ty = self.vars
                                        .get(i)
                                        .unwrap_or_else(|| abort!(e, "Needs to be a known variable"))
                                        .clone();

                                    let ind_expr = self.fold_expr(*ind.index);
                                    left = parse_quote!(#crate_root::core::operations::index::index(&#i, #ind_expr));
                                    
                                    let ind_ty = self.return_type.clone();

                                    quote!(<#ty as #crate_root::core::type_traits::IndexAble>::IndexResult, #crate_root::core::operation::OperationWrapper<<#ty as #crate_root::core::type_traits::IndexAble>::IndexResult, #crate_root::core::operations::index::Index<<#ty as #crate_root::core::type_traits::IndexAble>::IndexResult, #ty, #ind_ty>>)
                                } else {
                                    abort!(ind, "Not a variable")
                                }
                            }
                            _ => {
                                abort!(e, "Needs to be a local variable 2")
                            }
                        };

                        let old = self.block_prev.clone();
                        let old_expr = self.block_prev_type.clone();

                        self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                            #crate_root::core::types::Void, 
                            #old_expr,
                            #prev, 
                            #old
                        >);

                        self.return_type = quote!(#crate_root::core::operation::OperationWrapper<
                                #crate_root::core::types::Void,  
                                #crate_root::core::operations::set::Set<#ret, #after>
                            >
                        );
                        self.expr_type = quote!(#crate_root::core::types::Void);

                        //eprintln!("ExprAssign");
                            parse_quote! {
                                #crate_root::core::operations::set::set(&#left, #right)
                            }
                        
                    }
                    Expr::ForLoop(x) => {
                       match *x.pat {
                            Pat::Ident(p) => {
                                let prev = self.return_type.clone();

                                let right_side = self.fold_expr(*x.expr);
                                let iterable_ty = self.return_type.clone();

                                self.return_type = prev.clone();

                                let exprty = self.expr_type.clone(); 

                                self.vars.insert(p.clone().ident, quote!(#exprty));

                                let before = self.block_prev.clone();
                                let before_expr = self.block_prev_type.clone();
                                self.block_prev = quote!(#crate_root::core::operations::noop::Noop);
                                self.block_prev_type = quote!(#crate_root::core::types::Void);

                                let block = self.fold_block(x.body.clone());

                                let scope_type = self.return_type.clone();

                                self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                                    #crate_root::core::types::Void, 
                                    #before_expr, 
                                    #prev, 
                                    #before
                                >);

                                self.return_type = quote!(
                                    #crate_root::core::operation::OperationWrapper<
                                        #crate_root::core::types::Void, 
                                        #crate_root::core::operations::foreach::ForEach<
                                            #exprty, 
                                            #iterable_ty, 
                                            <#scope_type as #crate_root::core::operations::scope::ScopeTrait>::A, 
                                            <#scope_type as #crate_root::core::operations::scope::ScopeTrait>::B
                                        >
                                    >
                                );

                                parse_quote!{
                                    #crate_root::core::operations::foreach::foreach(#right_side, |#p| {
                                        #block
                                    })
                                }   
                            }
                            any => abort!(any, "Only support variables in for loop")
                        }  
                    }
                    Expr::If(x) => {
                        let prev = self.return_type.clone();

                        let condition = self.fold_expr(*x.cond);
                        let cond_ty = self.return_type.clone();

                        let before = self.block_prev.clone();
                        let before_expr = self.block_prev_type.clone();
                        self.block_prev = quote!(#crate_root::core::operations::noop::Noop);
                        self.block_prev_type = quote!(#crate_root::core::types::Void);

                        let block = self.fold_block(x.then_branch.clone());
                        let block_ty = self.return_type.clone();
                        let block_expr = self.expr_type.clone();

                        self.return_type = prev.clone();

                        if let Some((_, els_raw)) = x.else_branch {
                            self.block_prev = quote!(#crate_root::core::operations::noop::Noop);
                            self.block_prev_type = quote!(#crate_root::core::types::Void);

                            let els = self.fold_expr(*els_raw);
                            let els_ty = self.return_type.clone();
                            let els_expr = self.expr_type.clone();

                            self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                                #crate_root::core::types::Void, 
                                #before_expr, 
                                #prev, 
                                #before
                            >);
                        
                            self.return_type = quote!(
                                #crate_root::core::operation::OperationWrapper<
                                    #block_expr,
                                    #crate_root::core::operations::if_else::IfElse<
                                        #block_expr, 
                                        #cond_ty, 
                                        <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::A,
                                        <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::B,
                                        <#els_ty as #crate_root::core::operations::scope::ScopeTrait>::A,
                                        <#els_ty as #crate_root::core::operations::scope::ScopeTrait>::B,
                                    >
                                >
                            );
                        
                            parse_quote!{
                                #crate_root::core::operations::if_else::if_then_or_else(#condition, #block, #els)
                            }   
                        } else {
                            self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                                #crate_root::core::types::Void, 
                                #before_expr, 
                                #prev, 
                                #before
                            >);

                            self.return_type = quote!(
                                #crate_root::core::operation::OperationWrapper<
                                    #block_expr,
                                    #crate_root::core::operations::if_else::IfElse<
                                        #block_expr, 
                                        #cond_ty, 
                                        <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::A,
                                        <#block_ty as #crate_root::core::operations::scope::ScopeTrait>::B,
                                        #crate_root::core::operations::noop::Noop,
                                        #crate_root::core::operations::noop::Noop,
                                    >
                                >
                            );
                        
                            parse_quote!{
                                #crate_root::core::operations::if_else::if_then(#condition, #block)
                            }   
                        }
                    }
                    Expr::Block(b) => {
                        let res = self.fold_block(b.block);

                        parse_quote!{
                            #res
                        }
                    }
                    Expr::Return(r) => {
                        let prev = self.return_type.clone();

                        let exp = if let Some(x) = r.expr.clone() {
                            self.fold_expr(*x)
                        } else {
                            self.expr_type = quote!(#crate_root::core::types::Void);
                            self.return_type = quote!(#crate_root::core::operations::noop::Noop);
                            parse_quote!(#crate_root::core::operations::noop::Noop)
                        };

                        let after = self.return_type.clone();
                        let ret = self.expr_type.clone();

                        let old = self.block_prev.clone();
                        let old_expr = self.block_prev_type.clone();

                        self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                            #ret, 
                            #old_expr,
                            #prev, 
                            #old
                        >);

                        self.return_type = quote!(
                            #crate_root::core::operations::returns::Returns<#ret, #after>
                        );

                        return parse_quote! {
                            let #scope = #scope.returns(#crate_root::core::operations::returns::returns(#exp));
                        }
                    }

                    Expr::Struct(s) => {
                        if let Some(_) = semi.clone() {
                            abort!(s, "Currently structs are only allowed as last return expression")
                        }

                        let prev = self.return_type.clone();

                        let ret = self.expr_type.clone();

                        let old = self.block_prev.clone();
                        let old_expr = self.block_prev_type.clone();

                        self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                            #ret, 
                            #old_expr,
                            #prev, 
                            #old
                        >);

                        self.returns_struct = true;

                        let path = s.path.clone();

                        let mut stream = TokenStream::new();

                        for field in s.fields {
                            let assignment = self.fold_expr(field.expr);
                            let member = field.member;

                            stream.extend(quote!(#member: #assignment,))
                        }

                        self.return_type = quote!(
                            #path
                        );

                        parse_quote!(#path {#stream} )
                    }

                    Expr::Cast(c) => {
                        if let Some(_) = semi.clone() {
                            abort!(c, "Currently cast expresions are only allowed as last return expression")
                        }

                        let prev = self.return_type.clone();

                        let ret = self.expr_type.clone();

                        let old = self.block_prev.clone();
                        let old_expr = self.block_prev_type.clone();

                        self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                            #ret, 
                            #old_expr,
                            #prev, 
                            #old
                        >);

                        let ty = c.ty;
                        self.expr_type = quote!(#ty);
                        let res = self.fold_expr(*c.expr);
                        self.expr_type = quote!(#ty);
                        res
                    }

                    any => {
                        let prev = self.return_type.clone();

                        let res_ty = self.expr_type.clone();

                        let before = self.block_prev.clone();
                        let before_expr = self.block_prev_type.clone();
                        self.block_prev = quote!(#crate_root::core::operations::noop::Noop);

                        let res = self.fold_expr(any.clone());

                        //println!();
                        //println!("{:?}", any);
                        //println!("Expr Any: ");
                        //println!("Before: {}", before);
                        //println!();
                        //println!();

                        self.block_prev = quote!(#crate_root::core::operations::instruction_list::InstructionList<
                            #crate_root::core::types::Void, 
                            #before_expr, 
                            #prev, 
                            #before
                        >);

                        res
                    },
                };

                // let interet = self.return_type;

                // self.return_type = quote!(#crate_root::core::operations::set::Set)

                //eprintln!("Semi");
                if let Some(_) = semi {
                    parse_quote! {
                        let #scope = #scope.include(#exp);
                    }
                } else {
                    parse_quote! {
                        let #scope = #scope.returns(#exp);
                    }
                }
                
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}
