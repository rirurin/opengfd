use proc_macro::TokenStream;

#[proc_macro]
pub fn create_gfd_static(input: TokenStream) -> TokenStream {
    opengfd_proc_impl::create_gfd_static(input.into()).into()
}
