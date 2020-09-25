#[proc_macro_attribute]
pub fn implements(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _attribute = syn::parse_macro_input!(attribute as ImplementsAttribute);
    let _input = syn::parse_macro_input!(input as syn::ItemStruct);

    proc_macro::TokenStream::new()
}

struct ImplementsAttribute {}

impl syn::parse::Parse for ImplementsAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        loop {
            input.parse::<syn::UseTree>()?;

            if input.parse::<syn::Token!(,)>().is_err() {
                break;
            }
        }

        Ok(Self{})
    }
}
