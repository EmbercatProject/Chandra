use std::collections::HashMap;

use proc_macro_error::abort_call_site;
use quote::{format_ident};
use syn::{parse_quote, Path};


#[derive(Debug)]
pub struct ParseAttributes {
    pub structures: HashMap<syn::TypePath, Structure>,
    pub replace_functions: HashMap<syn::Path,syn::Path>,
    pub normal_functions: HashMap<syn::Path,syn::Path>,
}

pub type Structure = HashMap<String, String>;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Struct,
    Use,
    As
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Keyword(Keyword),
    AngleOpen,
    AngleClose,
    CurlyOpen,
    CurlyClose,
    Semicolon,
    Comma,
    Colon, 
    Plus, 
    Unsupported(char),
    None
} 

impl Token {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Token::AngleClose,
            '<' => Token::AngleOpen,
            '{' => Token::CurlyOpen,
            '}' => Token::CurlyClose,
            ':' => Token::Colon,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '+' => Token::Plus,
            x   => Token::Unsupported(x)
        }
    }

    fn from_string(s: String) -> Self {
        match s.as_str() {
            "struct" => Token::Keyword(Keyword::Struct),
            "use" => Token::Keyword(Keyword::Use),
            "as" => Token::Keyword(Keyword::As),
            x => Token::Ident(x.to_string())
        }
    }
}

impl ParseAttributes {

    pub fn new() -> Self {
        Self {
            structures: HashMap::new(),
            replace_functions: HashMap::new(),
            normal_functions: HashMap::new(),
        }
    }

    pub fn parse(&mut self, code: String) {
        let mut expressions = Vec::new();
        let mut tokens = Vec::new();

        let mut current = String::new();
        for cha in code.chars() {
            match cha {
                x if char::is_whitespace(x) => {
                    if current == "" {
                        continue;
                    } else {
                        tokens.push(Token::from_string(current.to_owned()));
                        current = String::new();
                    }
                }
                ';' => {
                    if current != "" {
                        tokens.push(Token::from_string(current.to_owned()));
                    } 
                        tokens.push(Token::from_char(cha));
                        expressions.push(tokens);

                        tokens = Vec::new();
                        current = String::new();
                    
                }
                ':' | ',' | '+' | '{' | '}' => {
                    if current != "" {
                        tokens.push(Token::from_string(current.to_owned()));
                    } 
                        tokens.push(Token::from_char(cha));
                        current = String::new();
                    
                }
                x => {
                    current = current + &x.to_string();
                }
            }
        }

        for expression in expressions.into_iter() {
            let mut valid = true;
            let mut last = Token::None;

            let mut it = expression.clone().into_iter();

            let token = it.next().unwrap_or_else(|| abort_call_site!("Expect a token"));

            match token {
                Token::Keyword(k) => {
                    match k {
                        Keyword::Struct => {
                            let mut structure = Structure::new();
                            let ident = it.next().unwrap_or_else(|| abort_call_site!("No ident"));
                            let ident = if let Token::Ident(x) = ident {
                                x
                            } else {
                                abort_call_site!("Not an ident: {:?}", ident)
                            };

                            let mut next = it.next().unwrap_or_else(|| abort_call_site!("Empty Struct"));

                            if Token::AngleOpen == next {
                                next = it.next().unwrap_or_else(|| abort_call_site!("Unexpected End of input"));

                                while next != Token::AngleClose {
                                    next = it.next().unwrap_or_else(|| abort_call_site!("Unexpected End of input"));
                                }
                                next = it.next().unwrap_or_else(|| abort_call_site!("Unexpected End of input"));

                                //let genericIdent = if let Token::Ident(x) = next {
                                //    x
                                //} else {
                                //    abort_call_site!("Not an ident: {:?}", ident)
                                //};


                            };

                            if Token::CurlyOpen != next {
                                abort_call_site!("Need curly open")
                            }

                            while next != Token::CurlyClose {
                                let ident = it.next().unwrap_or_else(|| abort_call_site!("No ident"));
                                let colon = it.next().unwrap_or_else(|| abort_call_site!(&format!("{:?} No Colon / expr: {:?}", ident, expression)));
                                let typ = it.next().unwrap_or_else(|| abort_call_site!("No Type"));

                                next = it.next().unwrap_or_else(|| abort_call_site!("Unexpected End of input"));

                                if let (Token::Ident(id), Token::Colon, Token::Ident(t)) = (ident, colon, typ) {
                                    structure.insert(id, t);
                                } else {
                                    abort_call_site!("Bad input")
                                }
                            }

                            let ident_t = format_ident!("{}", ident);
                            
                            self.structures.insert(parse_quote!(#ident_t), structure);
                        },
                        Keyword::Use => {
                            let mut next = it.next().unwrap_or_else(|| abort_call_site!("No Path after use"));

                            let mut path = String::new();
                            let mut as_replacement = false;
                            let mut replacing = String::new();
                            while next != Token::Semicolon {
                                if let Token::Keyword(Keyword::As) = next {
                                    if path.is_empty() {
                                        abort_call_site!("Need a path before applying 'as' to an import");
                                    }
                                    as_replacement = true;
                                    next = it.next().unwrap_or_else(|| abort_call_site!("Replacement Path after 'as' is missing"));
                                    continue;
                                }

                                let input = match &next {
                                    Token::Colon => ":",
                                    Token::Ident(id) => id,
                                    _ => abort_call_site!("{:?} not supported in path", next),
                                };

                                if as_replacement {
                                    replacing += &input;
                                } else {
                                    path += &input;
                                }

                                next = it.next().unwrap_or_else(|| abort_call_site!("Unexpected EOF"));
                            }

                            let p_ident = syn::parse_str::<Path>(&path).unwrap_or_else(|x| abort_call_site!(format!("{}, Not a valid path", x)));

                            if as_replacement {
                                let r_ident = syn::parse_str::<Path>(&replacing).unwrap_or_else(|x| abort_call_site!(format!("{}, Not a valid path", x)));

                                self.replace_functions.insert(parse_quote!(#r_ident), parse_quote!(#p_ident));
                            } else {
                                self.normal_functions.insert(parse_quote!(#p_ident), parse_quote!(#p_ident));
                            }
                        },
                        Keyword::As => abort_call_site!("As should not be a starting point")
                    }
                },
                x => abort_call_site!("Not a good start: {:?}", x)
            }
            
        }
    }
}