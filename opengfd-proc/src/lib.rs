use proc_macro::TokenStream;

#[proc_macro]
pub fn create_gfd_static(input: TokenStream) -> TokenStream {
    opengfd_proc_impl::gfd_static::create_gfd_static(input.into()).into()
}

#[proc_macro_derive(GfdRcAuto, attributes(Rc))]
pub fn gfd_rc_type_derive(item: TokenStream) -> TokenStream {
    opengfd_proc_impl::derive_rc::gfd_rc_type_derive(item.into()).into()
}
