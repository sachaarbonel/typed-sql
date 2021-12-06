use proc_macro::TokenStream;
// use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use nom_sql::sql_query;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // macro input must be `LitStr`, which is a string literal.
    // if not, a relevant error message will be generated.
    // let input = parse_macro_input!(input as LitStr);

    // get value of the string literal.
    let str_value = input.to_string();
    let res  = sql_query(str_value.as_bytes());
    let res_unwrap =  res.unwrap().1.to_string();
    
    // do something with value...
    // let str_value = str_value.to_uppercase();

    // generate code, include `str_value` variable (automatically encodes
    // `String` as a string literal in the generated code)
    (quote! {
        #res_unwrap
    })
    .into()
}
