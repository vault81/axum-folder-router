use prettyplease;

// helper for outputting TokenStream snippets
pub fn prettyprint(item: proc_macro::TokenStream) -> String {
    if let Ok(item) = syn::parse(item.clone()) {
        let file = syn::File {
            attrs:   vec![],
            items:   vec![item],
            shebang: None,
        };

        prettyplease::unparse(&file)
    } else {
        format!("{}", item)
    }
}
