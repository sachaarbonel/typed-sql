use proc_macro::TokenStream;
use quote::quote;

use nom_sql::{sql_query, parse_query};

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let str_value = input.to_string();
    let res = parse_query(str_value).unwrap();
    let res_unwrap = res.to_string();
    (quote! {
        #res_unwrap
    })
    .into()
}
