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

pub fn kernel(attr: TokenStream, tokens: TokenStream, crate_root: TokenStream2) -> TokenStream {
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

            let mut final_input_types = Vec::new();
            let mut final_input_names = Vec::new();
            let mut final_input_position = Vec::new();
            let mut position_counter: u8 = 0;

            let mut final_input_names_str = Vec::new();
            let mut final_output_type = TokenStream2::new();
            let mut final_output_name = TokenStream2::new();
            let mut final_output_name_str = String::new();
            
            for inp in inputs.clone().iter() {
                match inp {
                    FnArg::Typed(p) => {
                        if let Pat::Ident(i) = *p.pat.clone() {
                            let ty = &p.ty;

                            if let Type::Path(t_p) = ty.as_ref() {

                                input_vars.insert(i.ident.clone(), quote!(#crate_root::core::operations::var::Variable::<#t_p>));

                                if let Some(known) = parser.structures.get(&t_p) {
                                    input_structures.insert(i.ident, (*known).clone());
                                } else {
                                    let id_l = i.ident;
                                    let t_name = format!("{}", id_l);
                                    input_known_base_types.push(quote!(let #id_l = #crate_root::core::operations::var::Variable::<#t_p>::new(#t_name);));
                                }
                            } else if let Type::Reference(t_r) = ty.as_ref() {
                                if let Type::Path(t_p) = t_r.elem.as_ref() {

                                    input_vars.insert(i.ident.clone(), quote!(#crate_root::core::operations::var::Variable::<#t_p>));
                                    
                                    if let Some(_) = t_r.mutability {
                                        let id = i.ident.clone();
                                        final_output_type = quote!(#t_p);
                                        final_output_name = quote!(#id);
                                        final_output_name_str = format!("{}", id);
                                    } else {
                                        final_input_types.push(t_p.clone());
                                        final_input_names.push(i.ident.clone());
                                        final_input_names_str.push(format!("{}", i.ident.clone()));
                                        
                                        final_input_position.push(position_counter);
                                        position_counter +=1;
                                    }
                                    

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
                expr_type: quote!(#result), 
                var_levels: Vec::new(), 
                vars: input_vars, 
                scope_depth: 0, 
                known_structures: input_structures, 
                block_prev: quote!(#crate_root::core::operations::noop::Noop),
                block_prev_type: quote!(#crate_root::core::types::Void),
                known_generics,
                crate_root: crate_root.clone(),
                returns_struct: false,
                known_extensions: parser.replace_functions.clone(),
                known_functions: parser.normal_functions.clone(),
            };

            let public = func.vis;

            let b = p.fold_block(*func.block.clone());
            let mut cpu_p = ParseCPUfn { known_functions: parser.normal_functions };

            let block = cpu_p.fold_block(*func.block);

            if p.returns_struct {
                abort!(block, "Only ChandraExtensions are allowed to return a structure. Kernels must not return a value")
            }

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

            let cpu_fn_ident = format_ident!("{}CPUFn", ident.to_string().to_case(Case::UpperCamel));
            let executable_inputs_ident = format_ident!("{}Inputs", ident.to_string().to_case(Case::UpperCamel));
            let function_struct_ident = format_ident!("{}", ident.to_string().to_case(Case::UpperCamel));
            let return_type_ident = format_ident!("{}Programm", ident.to_string().to_case(Case::UpperCamel));

            let mut inner_generics = func_generics.params.clone();
            inner_generics_to_static_bound(&mut inner_generics);
            let phantom_data_idents: Vec<Ident> = inner_generics.iter().enumerate().map(|(i, _)| format_ident!("_{}", i)).collect();

            let inner_generics_idents = inner_generics_idents(&inner_generics);
            

            quote!{
                #public type #return_type_ident<#inner_generics> = #fn_return;

                #public struct #function_struct_ident <#inner_generics> {
                    main: #return_type_ident<#(#inner_generics_idents,)*>,
                    cpu_fn: #cpu_fn_ident,
                    #(#phantom_data_idents: ::core::marker::PhantomData<#inner_generics_idents>,)*
                }

                impl<#inner_generics> #function_struct_ident<#(#inner_generics_idents,)*> {
                    #public fn build <P: #crate_root::core::processor::Processor> (self, processor: &mut P) -> <P as #crate_root::core::processor::ProcessorInformation>::Executable<#function_struct_ident <#(#inner_generics_idents,)*>>
                    #processor_mem_bounds 
                    #return_type_ident<#(#inner_generics_idents,)*>: #crate_root::core::operation::Compilable<#crate_root::core::types::Void, P::Compiler>,
                    P::Executable<#function_struct_ident <#(#inner_generics_idents,)*>>: #crate_root::core::processor::Executable<P::Storage, #executable_inputs_ident<P::Storage, #(#inner_generics_idents,)*>> {
                        processor.build(self)
                    }
                }

                impl<P: #crate_root::core::processor::ProcessorInformation, #inner_generics> #crate_root::core::Buildable<P> for #function_struct_ident <#(#inner_generics_idents,)*> 
                #processor_mem_bounds 
                #return_type_ident<#(#inner_generics_idents,)*>: #crate_root::core::operation::Compilable<#crate_root::core::types::Void, P::Compiler> {
                    type Binding = #executable_inputs_ident <P::Storage, #(#inner_generics_idents,)*>;
                    type CPUBinding = #executable_inputs_ident <#crate_root::processor::cpu::CPUStorage, #(#inner_generics_idents,)*>;
                    type CPUFunction = #cpu_fn_ident;
                
                    type Main = #return_type_ident<#(#inner_generics_idents,)*>;
                
                    fn get_cpu(&self) -> Self::CPUFunction {
                        self.cpu_fn
                    }
                
                    fn get_main_tree(&self) -> Self::Main {
                        self.main.clone()
                    }
                }

                impl<#inner_generics> #crate_root::core::Program for #function_struct_ident <#(#inner_generics_idents,)*> {
                    type MainTree = #return_type_ident<#(#inner_generics_idents,)*>;
                
                    fn get_main_tree(&self) -> Self::MainTree {
                        self.main.clone()
                    }
                }

                #public struct #executable_inputs_ident <S: #crate_root::core::processor::Storage, #inner_generics> #mem_bounds {
                    #(#final_input_names: ::std::option::Option< #crate_root::core::allocated::Binding<S, #final_input_types>>,)*
                    #final_output_name: ::std::option::Option< #crate_root::core::allocated::Binding<S, #final_output_type>>
                }

                impl<S: #crate_root::core::processor::Storage, #inner_generics> #executable_inputs_ident<S, #(#inner_generics_idents,)*> #mem_bounds {
                    #public fn bind(&mut self, #(#final_input_names: &#crate_root::core::allocated::Binding<S, #final_input_types>,)* #final_output_name: &mut #crate_root::core::allocated::Binding<S, #final_output_type>) {
                        #(self.#final_input_names = ::std::option::Option::Some(#final_input_names.clone());)*
                        self.#final_output_name = ::std::option::Option::Some(#final_output_name.clone())
                    }
                }

                impl<S: #crate_root::core::processor::Storage, #inner_generics> #crate_root::core::allocated::ExecutableBindings<S> for #executable_inputs_ident<S, #(#inner_generics_idents,)*> #mem_bounds {
                    type CPU = #executable_inputs_ident<#crate_root::processor::cpu::CPUStorage, #(#inner_generics_idents,)*>;
                    type I = (#(#crate_root::core::allocated::Binding<S, #final_input_types>,)*);
                    type O = #final_output_type;
                    
                    fn new() -> Self {
                        Self {
                            #(#final_input_names: ::std::option::Option::None,)*
                            #final_output_name: ::std::option::Option::None
                        }
                    }
                
                    fn get_inputs(&self) -> Self::I {
                        (#(self.#final_input_names.clone().unwrap(),)*)
                    }
                
                    fn get_output(&self) -> #crate_root::core::allocated::Binding<S, Self::O> {
                        self.#final_output_name.clone().unwrap()
                    }

                    fn get_input_references(&self) -> Vec<<S as #crate_root::core::processor::Storage>::Key> {
                        vec![#(self.#final_input_names.as_ref().unwrap().get_reference(),)*]
                    }
            
                    fn get_out_reference(&self) -> <S as #crate_root::core::processor::Storage>::Key {
                        self.#final_output_name.as_ref().unwrap().get_reference()
                    }

                    fn get_layouts() -> ::std::collections::HashMap<String, (u8, #crate_root::core::allocated::MemoryLayoutDescriptor, bool)> {
                        ::std::collections::HashMap::from([
                            #((#final_input_names_str.to_string(), (#final_input_position, <#final_input_types as #crate_root::core::type_traits::Generalizable>::get_memory_layout(), false)),)*
                            (#final_output_name_str.to_string(), (0, <#final_output_type as #crate_root::core::type_traits::Generalizable>::get_memory_layout(), true)),
                        ])
                    }
                }          

                #[derive(::core::marker::Copy, ::core::clone::Clone, ::std::fmt::Debug)]
                #public struct #cpu_fn_ident;

                impl<#inner_generics> #crate_root::core::processor::cpu::CPUFunction<#executable_inputs_ident<#crate_root::processor::cpu::CPUStorage, #(#inner_generics_idents,)*>> for #cpu_fn_ident {
                    fn call_cpu<'a, 'b>(&self, pos: & #crate_root::core::types::Pos, (#(#final_input_names,)*): &(#( &#final_input_types,)*) , #final_output_name: &mut <<#final_output_type as #crate_root::core::type_traits::MemoryMapable<#crate_root::processor::cpu::CPUStorage>>::Mapped<'b> as FromMut<<#crate_root::processor::cpu::CPUStorage as #crate_root::core::processor::Storage>::MappedType<#final_output_type>>>::Result<'b>) 
                        #block
                }

                //fn test_fn #func_generics(#(#input_idents: #types,)*) #block

                //fn test_memable #mem_generic (#mem_imputs) {
//
                //}

                /// I am a cool doc comment
                /// 
                #public fn #ident <#inner_generics> () -> #function_struct_ident<#(#inner_generics_idents,)*> {
                    
                    #(#parsed_strucures)*

                    #(#input_known_base_types)*
                    
                    let __build_apl = || #b;
                    
                    #function_struct_ident {
                        main: __build_apl(),
                        cpu_fn: #cpu_fn_ident,
                        #(#phantom_data_idents: ::core::marker::PhantomData,)*
                    }
                }

            }
        }
        any => abort!(any, "macro needs to be placed above a function"),
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
