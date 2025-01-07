use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

struct ReferenceField<'a> {
    ast: &'a syn::Field,
    has_attribute: bool
}

// opengfd::utility::reference::Reference
static REFERENCE_PATH: [&'static str; 4] = [ "opengfd", "utility", "reference", "Reference" ];

fn is_path_correct(path: &syn::Path) -> bool {
   for (i, segment) in path.segments.iter().rev().enumerate() {
       if segment.ident.to_string() != REFERENCE_PATH[3 - i] {
           return false;
       }
   }
   true
}

fn get_reference_fields(item: &syn::ItemStruct) -> syn::Result<ReferenceField> {
    match &item.fields {
        syn::Fields::Named(n) => {
            let mut ref_fields = vec![];
            let mut got_rc_attribute = false;
            for field in &n.named {
                match &field.ty {
                    syn::Type::Path(p) => {
                        if p.path.get_ident().is_none() {
                            continue; // we need a concrete type name
                        }
                        let rc_attrib: Vec<&syn::Attribute> = field.attrs.iter().filter(|f| f.path().is_ident("Rc")).collect();
                        let has_attribute = match rc_attrib.len() {
                            0 => false,
                            1 => {
                                if got_rc_attribute {
                                    return Err(syn::Error::new(Span::call_site(), "Rc field attribute can only be used once per struct"))
                                }
                                got_rc_attribute = true;
                                true
                            },
                            _ => return Err(syn::Error::new(Span::call_site(), "Rc field attribute can only be used once per field"))
                        };
                        if !is_path_correct(&p.path) { continue; }
                        ref_fields.push(ReferenceField {
                            ast: field, has_attribute
                        });
                    },
                    _ => continue
                };
            }
            // We will allow multiple References per struct, so long as the right one is annotated
            match ref_fields.len() {
                0 => Err(syn::Error::new(Span::call_site(), "No reference type found")),
                1 => Ok(ref_fields.remove(0)),
                _ => Ok(ref_fields.remove(ref_fields.iter().position(|f| f.has_attribute).unwrap()))
            }
        },
        _ => Err(syn::Error::new(Span::call_site(), "GfdRcType can only be derived on named structs"))
    }
}

fn build_derive_impl(field: &ReferenceField, item: &syn::ItemStruct) -> syn::Result<TokenStream> {
    let struct_name = syn::Ident::new(&item.ident.to_string(), Span::call_site());
    let field_name = syn::Ident::new(&field.ast.ident.as_ref().unwrap().to_string(), Span::call_site());
    let impl_start = match item.generics.params.len() {
        0 => quote! { impl GfdRcType for #struct_name },
        _ => {
            // Even if the generic contains a where clause, it's not added to the token
            // stream when to_token_stream is called. This is probably a bug!
            let mut generics_impl = item.generics.clone();
            for generic_arg in &mut generics_impl.params {
                match generic_arg {
                    syn::GenericParam::Type(t) => {
                        // remove the default argument, we don't need that
                        t.eq_token = None;
                        t.default = None;
                    },
                    _ => continue
                }
            }
            let generics_impl = generics_impl.to_token_stream();
            let generics_impl_full = match &item.generics.where_clause {
                Some(v) => {
                    let where_clause = v.to_token_stream();
                    quote! { #generics_impl #where_clause }
                },
                None => generics_impl.clone()
            };
            quote! { impl #generics_impl GfdRcType for #struct_name #generics_impl_full }
        }
    };
    let body = quote! {
        {
            fn count(&self) -> u32 { self.#field_name.count() }
            fn add_ref(&self) -> u32 { self.#field_name.add_ref() }
            fn release(&self) -> u32 { self.#field_name.release() }
        }
    };
    Ok(quote! {
        #impl_start
        #body
    })
}

pub fn gfd_rc_type_derive(item: TokenStream) -> TokenStream {
    let target_item: syn::ItemStruct = match syn::parse2(item) {
        Ok(n) => n,
        Err(e) => return TokenStream::from(e.to_compile_error())
    };
    let field = match get_reference_fields(&target_item) {
        Ok(n) => n,
        Err(e) => return TokenStream::from(e.to_compile_error())
    };
    let tokens = match build_derive_impl(&field, &target_item) {
        Ok(n) => n,
        Err(e) => return TokenStream::from(e.to_compile_error())
    };
    // println!("{}", tokens.to_string());
    tokens
}

// impl GfdRcType for #StructName 
// fn count(&self) -> u32 { self.#ref.count() }
// fn add_ref(&self) -> u32 { self.#ref.add_ref() }
// fn release(&self) -> u32 { self.#ref.release() } 
