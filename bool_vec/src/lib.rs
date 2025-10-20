extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

enum MyErr {
    NotBool,
}


fn bool_vec_impl(input: TokenStream) -> Result<TokenStream, MyErr> {
    //let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut result = "vec![".to_string();
    let mut chars = input.into_iter().filter_map(|token| {
        if let proc_macro::TokenTree::Literal(literal) = token{
            return Some(literal);
        }
        None
    }).map(|literal|{format!("{}", literal).chars().collect::<Vec<_>>()})
    .flatten();
    match chars.next(){
        Some(first) => {
            result = format!("{}{}", result, {
                match first{
                    '0' => false,
                    '1' => true,
                    _ => return Err(MyErr::NotBool)
                }
            }
        )
        }
        None => return Ok("vec![]".parse().unwrap()),
    }
    for symb in chars{
            match symb{
                '0' => result = format!("{},{}", result, false),
                '1' => result = format!("{},{}", result, true),
                '_' => {},
                _ => return Err(MyErr::NotBool)
            }
    }
    result = format!("{}]", result);
    Ok(result.parse().unwrap())
    //Ok([TokenTree::Literal(Literal::string(&result))].into_iter().collect())
}

#[proc_macro]
pub fn bool_vec(input: TokenStream) -> TokenStream {
    match bool_vec_impl(input) {
        Ok(v) => v,
        Err(_) => [
            TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                [TokenTree::Literal(Literal::string("Bool vector is created from 0s and 1s!"))].into_iter().collect(),
            )),
        ]
        .into_iter()
        .collect(),
    }
}