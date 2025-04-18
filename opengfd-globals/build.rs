use std::{
    error::Error,
    path::{ Path, PathBuf }
};
use proc_macro2::Span;
use opengfd_proc_impl::gfd_static::create_gfd_static_links;
use syn;
use quote::ToTokens;

fn get_global_file_relative() -> &'static str {
    #[cfg(feature = "v1-core")]
    let out = "src/globals_xrd744.rs";
    #[cfg(feature = "v2-core")]
    let out = "src/globals_xrd759.rs";
    out
}

fn get_or_make_child_dir<T: AsRef<Path>>(d: T, c: &str) -> Result<PathBuf, Box<dyn Error>> {
    let out = d.as_ref().join(c);
    if !out.exists() { std::fs::create_dir(&out)?; }
    Ok(out)
}

// #[link(name = "opengfd-globals", kind = "raw-dylib")]
// unsafe extern "C" {
//      unsafe fn set_ngr_crchash_vtable(ptr: *mut u8);
//      unsafe fn get_ngr_crchash_vtable() -> Option<&'static u8>;
//      unsafe fn get_ngr_crchash_vtable_mut() -> Option<&'static mut u8>;
//      unsafe fn get_ngr_crchash_vtable_unchecked() -> &'static u8;
//      unsafe fn get_ngr_crchash_vtable_unchecked_mut() -> &'static mut u8;
// }

fn generate_codegen_from_ast(mut source_ast: syn::File, to_self: bool) -> String {
    let mut output_file = format!("#![allow(dead_code, improper_ctypes)]
// This file was automatically generated from opengfd-globals.\n");
    for item in &mut source_ast.items {
        match item {
            syn::Item::Macro(m) => {
                if m.mac.path.is_ident("create_gfd_static") {
                    output_file.push_str(&create_gfd_static_links(m.mac.tokens.clone()));
                }
            },
            syn::Item::Use(u) => {
                // check that root of tree is for opengfd crate
                match &mut u.tree {
                    syn::UseTree::Path(p) => if &p.ident.to_string() == "opengfd" {
                        if to_self {
                            // we're linking this with OpenGFD itself, so replace opengfd in use with crate
                            p.ident = syn::Ident::new("crate", Span::call_site());
                        }
                        output_file.push_str(&u.to_token_stream().to_string());
                        output_file.push_str("\n");
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }
    output_file
}

fn save_codegen<P>(path: P, name: &str, output_file: String)
where P: AsRef<Path>
{
    let middata = get_or_make_child_dir(path.as_ref(), "middata").unwrap();
    let output_path = middata.join(name);
    std::fs::write(output_path, output_file).unwrap();
}

fn main() {
    let source_dir = std::env::current_dir().unwrap();
    println!("{}", format!("cargo::rerun-if-changed={}", get_global_file_relative()));
    println!("{}", format!("cargo::rerun-if-changed=build.rs"));
    let global_source = source_dir.join(get_global_file_relative());
    let source_ast = syn::parse_file(&std::fs::read_to_string(global_source).unwrap()).unwrap();
    let glb_self= generate_codegen_from_ast(source_ast.clone(), true);
    let glb_ext= generate_codegen_from_ast(source_ast, false);
    save_codegen(source_dir.clone(), "self.rs", glb_self);
    save_codegen(source_dir, "ext.rs", glb_ext);
}