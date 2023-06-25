use std::{collections::{HashMap}};

use convert_case::{Case, Casing};

use crate::{parseatt::Structure, parse_cpu_function::ParseCPUfn};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort, abort_call_site};
use quote::{format_ident, quote};
use syn::{fold::Fold, Item as SynItem,FnArg, Type, ReturnType, parse_quote, Ident, GenericParam, Pat, Path, punctuated::Punctuated, TypeParamBound};

use crate::parseatt::ParseAttributes;
use crate::parse_function::Parsefn;

pub fn function(attr: TokenStream, tokens: TokenStream, crate_root: TokenStream2, is_extension: bool) -> TokenStream {
    let description = proc_macro2::TokenStream::from(attr).to_string();

    let mut parser = ParseAttributes::new();
    parser.structures = get_default_structures();
    
    parser.parse(description);

   // println!("{:?}", parser);

    let tokens2 = proc_macro2::TokenStream::from(tokens);

    let parse2 = syn::parse2::<SynItem>(tokens2).unwrap_or_else(|_| abort_call_site!("Failed to parse tokens"));

    let res = match parse2 {
        SynItem::Fn(func) => {
            
            let sig = func.sig;
            let func_generics = sig.generics;
            let ident = sig.ident;

            let inputs: Vec<FnArg> = sig.inputs.clone().into_iter().collect();
            
            let types:Vec<Type> = inputs.clone().iter().map(|x| {
                match x {
                    FnArg::Typed(p) => *p.ty.clone(),
                    FnArg::Receiver(_) => abort!(x, "self not supported")
                }
            }).collect();

            let result = match sig.output {
                ReturnType::Type(_, t) => *t,
                ReturnType::Default => Type::Verbatim(quote!(()))
            };

            let mut known_generics = HashMap::new();

            for gen in func_generics.params.clone().into_iter() {
                match gen {
                    syn::GenericParam::Type(x) => {
                        let ident = x.ident;
                        known_generics.insert(ident.clone(), quote!(#ident));
                    }
                    any => abort!(any, "Only Type Generics are allowed")
                }
            }


            let mut input_vars = HashMap::new();
            let mut input_structures = HashMap::new();
            let mut input_known_base_types = Vec::new();

            let mut final_input_types: Vec<TokenStream2> = Vec::new();
            let mut final_input_names = Vec::new();
            let mut final_input_names_str = Vec::new();
            //let mut final_input_position = Vec::new();
            let mut position_counter: u8 = 0;

            let mut final_output_type = TokenStream2::new();
            let mut final_output_name = TokenStream2::new();
            let mut final_output_name_str = String::new();
            
            for inp in inputs.clone().iter() {
                match inp {
                    FnArg::Typed(p) => {
                        if let Pat::Ident(i) = *p.pat.clone() {
                            let ty = &p.ty;

                            if let Type::Path(t_p) = ty.as_ref() {

                                input_vars.insert(i.ident.clone(), quote!(#t_p));

                                final_input_types.push(quote!(#crate_root::core::operations::var::Variable<#t_p>));
                                final_input_names.push(i.ident.clone());
                                final_input_names_str.push(format!("{}", i.ident.clone()));

                                if let Some(known) = parser.structures.get(&t_p) {
                                    input_structures.insert(i.ident, (*known).clone());
                                } else {
                                    let id_l = i.ident;
                                    let t_name = format!("{}", id_l);
                                    input_known_base_types.push(quote!(let #id_l = #crate_root::core::operations::var::Variable::<#t_p>::new(#t_name);));
                                }
                            } else if let Type::Reference(t_r) = ty.as_ref() {
                                if let Type::Path(t_p) = t_r.elem.as_ref() {

                                    input_vars.insert(i.ident.clone(), quote!(#t_p));
                                    
                                    final_input_types.push(quote!(#crate_root::core::operations::var::Variable<#t_p>));
                                    final_input_names.push(i.ident.clone());
                                    final_input_names_str.push(format!("{}", i.ident.clone()));
                                    

                                   // println!();
                                   // println!("{:#?}", t_p);
                                   // println!();

                                    if let Some(known) = parser.structures.get(&t_p) {
                                        input_structures.insert(i.ident, (*known).clone());
                                    } else {
                                        let id_l = i.ident;
                                        let t_name = format!("{}", id_l);
                                        input_known_base_types.push(quote!(let #id_l = #crate_root::core::operations::var::Variable::<#t_p>::new(#t_name);));
                                    }
                                }
                            }

                        } else {
                            abort!(p, "Not an Ident")
                        }
                    }
                    FnArg::Receiver(_) => abort!(inp, "self not supported")
                }
            }

            let mut parsed_strucures = Vec::new();

            let mut struct_count: u32 = 0;

            for (ident, fields) in input_structures.iter() {
                let st = format_ident!("HelperStruct{}", struct_count);
                let mut field_types = Vec::new();
                let mut field_names = Vec::new();
                let mut access_names = Vec::new();

                for (name, typ) in fields.iter().map(|(k, v)| (format_ident!("{}", k), format_ident!("{}", v))) {
                    field_types.push(typ);
                    field_names.push(name.clone());
                    access_names.push(format!("{}.{}", ident, name));
                }

                let temp = quote!{
                    struct #st {
                        #(#field_names: #crate_root::core::operations::var::Variable<#field_types>,)*
                    }

                    let #ident = #st { 
                        #(#field_names: #crate_root::core::operations::var::Variable::<#field_types>::new(#access_names),)* 
                    };
                };

                parsed_strucures.push(temp);

                struct_count +=1;
            }

           // println!();
           // println!("{:#?}", input_vars);
           // println!();

            let mut p = Parsefn {
                return_type: quote!(#crate_root::core::operations::noop::Noop), 
                expr_type: quote!(#crate_root::core::types::Void), 
                var_levels: Vec::new(), 
                vars: input_vars, 
                scope_depth: 0, 
                known_structures: input_structures, 
                block_prev: quote!(#crate_root::core::operations::noop::Noop),
                block_prev_type: quote!(#crate_root::core::types::Void),
                known_generics,
                crate_root: crate_root.clone(),
                returns_struct: false,
                known_extensions: parser.replace_functions,
                known_functions: parser.normal_functions.clone(),
            };

            let public = func.vis;

            let b = p.fold_block(*func.block.clone());

            let fn_return = p.return_type;

            //let (mem_generic, mem_imputs) = get_mem_generics_and_types(types.clone());

            let mem_bounds = get_memmappable_bounds(types.clone(), parse_quote!(S), crate_root.clone());
            let processor_mem_bounds = get_memmappable_bounds(types.clone(), parse_quote!(P::Storage), crate_root.clone());

            //let mut docs = format!(" # Examples"); 
            //docs = format!("{} \n let prog = {}();", docs, ident);
//
            //for (i, t) in final_input_names.clone().iter().zip(final_input_types.clone()) {
            //    docs = format!("{} \n let {} = {};", docs, i, quote!(#t));
            //}

            let ident_str = ident.to_string();
            let cpu_fn_ident = format_ident!("{}CPUFn", ident.to_string().to_case(Case::UpperCamel));
            let executable_inputs_ident = format_ident!("{}Inputs", ident.to_string().to_case(Case::UpperCamel));
            let function_struct_ident = format_ident!("{}", ident.to_string().to_case(Case::UpperCamel));
            let return_type_ident = format_ident!("{}Programm", ident.to_string().to_case(Case::UpperCamel));

            let mut inner_generics = func_generics.params.clone();
            inner_generics_to_static_bound(&mut inner_generics);
            let phantom_data_idents: Vec<Ident> = inner_generics.iter().enumerate().map(|(i, _)| format_ident!("_{}", i)).collect();

            let inner_generics_idents = inner_generics_idents(&inner_generics);


            let mut impl_traits = TokenStream2::new();

            let cpu_fn = if is_extension {
                impl_traits = quote! {
                    impl<#inner_generics> #crate_root::core::type_traits::ChandraExtensionFn for #ident <#(#inner_generics_idents,)*> {
                        type Result = #result;
                        type Inputs = (#(#final_input_types,)*);
                        type FunctionScope = #fn_return;
                    }
                };
                quote!()
            } else if p.returns_struct {
                abort!(func.block, "Only ChandraExtensions are allowed to return a struct")
            } else {
                impl_traits = quote!{
                    impl<#inner_generics> #crate_root::core::type_traits::ChandraFn for #ident <#(#inner_generics_idents,)*> {
                        type Result = #result;
                        type Inputs = (#(#final_input_types,)*);
                        type FunctionScope = #fn_return;
                    }

                    impl<#inner_generics> #crate_root::core::type_traits::ChandraExtensionFn for #ident <#(#inner_generics_idents,)*> {
                        type Result = #result;
                        type Inputs = (#(#final_input_types,)*);
                        type FunctionScope = #fn_return;
                    }
                };

                let mut cpu_p = ParseCPUfn { known_functions: parser.normal_functions };

                let blo = cpu_p.fold_block(*func.block);
                quote!{
                    #public fn cpu(#(#inputs,)*) -> #result
                        #blo
                }
            };

            let mut doc = String::new();
            if is_extension {
                doc += "This is a ChandraExtension. This means it supports call and method call syntax. ";
            } else {
                doc += "This is a ChandraFunction. This means it only supports call syntax and can't be used as a method. ";
            }
            doc += &format!("You should use it only inside a [kernel] or [ChandraFunction].
                
# Example
```ignore
#[kernel{{
    use {};
}}]
fn my_kernel(pos: Pos, {} out: &mut Vec<{}>) {{
    out[pos.x] = {}({});
}}
```", if is_extension {format!("{} as {}", ident.to_string(), ident.to_string())} else {ident.to_string()}, quote!(#(#inputs,)*), quote!(#result), ident.to_string(), quote!(#(#final_input_names,)*));
            

            quote!{
                #[doc = #doc]
                #[allow(non_camel_case_types)]
                #public struct #ident <#inner_generics> {
                    value: #crate_root::core::operations::function::Function<#result, (#(#final_input_types,)*), #fn_return>
                }

                impl<#inner_generics> #ident <#(#inner_generics_idents,)*> {
                    #public fn build_fn() -> #crate_root::core::operations::function::Function<#result, (#(#final_input_types,)*), #fn_return> {
                        #(let #final_input_names = <#final_input_types>::new(#final_input_names_str);)*
                        
                        #crate_root::core::operations::function::function(#ident_str, (#(#final_input_names,)*), |(#(#final_input_names,)*)| #b) 
                    }

                    #cpu_fn
                }

                #impl_traits
            }
        }
        any => abort!(any, "Macro needs to be placed above a function"),
    }
    .into();

    //println!("{}", res);
    res
}

fn inner_generics_idents(generics: &Punctuated<GenericParam, syn::token::Comma>) -> Vec<Ident> {
    generics.
        iter()
        .map(|gen| {
            return match gen {
                GenericParam::Type(x) => {
                  x.ident.clone()
                }
                _ => abort!(gen, "Const and lifetime generics are not supported")
            }
        }).collect()
}

fn inner_generics_to_static_bound(befor: &mut Punctuated<GenericParam, syn::token::Comma>) {
    befor
        .iter_mut()
        .for_each(|gen| {
            match gen {
                GenericParam::Type(x) => {
                    x.bounds.push(TypeParamBound::Lifetime(parse_quote!('static)));
                }
                _ => abort!(gen, "Const and lifetime generics are not supported")
            }
        });
}

fn get_memmappable_bounds(types: Vec<Type>, mappable_path: Path, crate_root: TokenStream2) -> TokenStream2 {
    let mut has_mut = false;
    let mut unique_types = HashMap::<Path, Path>::new();

    for typ in types {
        match typ.clone() {
            Type::Reference(ref_type) => {
                let mutable = ref_type.mutability.is_some();
                if mutable {
                    if has_mut {
                        abort!(ref_type, "Can't have more than 1 mutable reference")
                    }
                    has_mut = true;
                }

                if let Type::Path(path_type) = *ref_type.elem {
                    if let Some(_) = unique_types.get(&path_type.path) {
                        continue;
                    } else  {
                        unique_types.insert(path_type.path, parse_quote!(#crate_root::core::type_traits::MemoryMapable<#mappable_path>));
                    }
                    //uniqueTypes.insert(pathType.path);
                } else {
                    abort!(ref_type, "Unsupported function argument Type")
                }

            }
            _ => continue
        }
    }

    let bounds: Vec<TokenStream2> = unique_types.into_iter().map(|(k,v)| quote!(#k: #v)).collect();

    quote!(where #(#bounds,)*)
}


fn get_mem_generics_and_types(types: Vec<Type>, crate_root: TokenStream2) -> (TokenStream2, TokenStream2) {
    let mut position_argument: TokenStream2 = TokenStream2::new();
    let mut has_mut = false;
    let mut unique_types = HashMap::<Path, Ident>::new();
    let mut gen_inputs: Vec<TokenStream2> = Vec::new();
    let mut input_count: u16 = 0;
    let mut count = 0_u16;

    for typ in types {
        match typ.clone() {
            Type::Path(path_type) => {
                let maybe_pos = path_type.path.segments.last().unwrap_or_else(|| abort!(typ, "Expect path to contain segments"));
                if maybe_pos.ident == "Pos" {
                    position_argument = quote!{pos: #typ}
                } else {
                    abort!(path_type, "All variables should be references with the exception of special variables")
                }
            }
            Type::Reference(ref_type) => {
                let mutable = ref_type.mutability.is_some();
                if mutable {
                    if has_mut {
                        abort!(ref_type, "Can't have more than 1 mutable reference")
                    }
                    has_mut = true;
                }

                if let Type::Path(path_type) = *ref_type.elem {
                    if let Some(gen) = unique_types.get(&path_type.path) {
                        let id = format_ident!("a{}_{}", input_count, gen);
                        
                        if mutable {
                            gen_inputs.push(quote!(#id: &mut #gen));
                        } else {
                            gen_inputs.push(quote!(#id: &#gen));
                        }
                        
                        input_count += 1;
                    } else  {
                        let gen = format_ident!("T{}", count);
                        unique_types.insert(path_type.path, gen.clone());

                        let id = format_ident!("a{}_{}", input_count, gen);
                        
                        if mutable {
                            gen_inputs.push(quote!(#id: &mut #gen));
                        } else {
                            gen_inputs.push(quote!(#id: &#gen));
                        }

                        input_count += 1;
                        count += 1;
                    }
                    //uniqueTypes.insert(pathType.path);
                } else {
                    abort!(ref_type, "Unsupported function argument Type")
                }
            }
            any => abort!(any, "Unsupported function argument Type")
        }
    }

    let generics: Vec<Ident> = unique_types.into_iter().map(|(_,v)| v).collect();

    return (
        quote!(<#(#generics: #crate_root::core::type_traits::Generalizable,)*>), 
        quote!(#position_argument, #(#gen_inputs,)*) )
}

fn get_default_structures() ->  HashMap<syn::TypePath, Structure> {
    let mut map = HashMap::new();

    map.insert(parse_quote!(Pos), HashMap::from([
        ("x".to_string(), "u32".to_string()),
        ("y".to_string(), "u32".to_string()),
        ("z".to_string(), "u32".to_string())
    ]));

    //map.insert(parse_quote!(Tensor<f32>), HashMap::new());

    map
}
