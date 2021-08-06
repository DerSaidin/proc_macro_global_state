#![forbid(unsafe_code)]

#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GLOBAL_STATE: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
}

#[proc_macro]
pub fn proc_macro_global_state_add(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut guard = GLOBAL_STATE.lock().unwrap();

    input.into_iter().for_each(|t| {
        eprintln!("TOKENS ADD:\n{}", t);
        let string_lit = match t {
            proc_macro::TokenTree::Literal(literal) => literal,
            _ => return,
        };
        guard.push(string_lit.to_string().trim_matches('"').to_string());
    });
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn proc_macro_global_state_emit(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let guard = GLOBAL_STATE.lock().unwrap();

    let result_token_stream = guard
        .iter()
        .fold(proc_macro2::TokenStream::new(), |mut s, t| {
            if !s.is_empty() {
                s.extend(quote! {,});
            }
            s.extend(proc_macro2::TokenStream::from(
                proc_macro2::TokenTree::Literal(proc_macro2::Literal::string(t)),
            ));
            s
        });
    eprintln!("TOKENS EMIT:\n{}", result_token_stream);
    let t = quote! { ( #result_token_stream ) };
    t.into()
}
