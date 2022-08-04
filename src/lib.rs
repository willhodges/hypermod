use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::result::Result;

#[proc_macro]
pub fn hypermod(_args: TokenStream) -> TokenStream {
    let src_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join("src"),
        None => PathBuf::from("src"),
    };
    let quote = match build_directory_mods_recursive(src_dir.as_path()) {
        Ok(expanded) => expanded,
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };
    quote.into()
}

fn build_directory_mods_recursive(directory: &Path) -> Result<TokenStream2, Error> {
    if !directory.is_dir() {
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    let mut files = Vec::new();
    let mut submodules = HashMap::new();

    for entry in directory.read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let directory = entry.path();
            let dirname = directory
                .clone()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            let submodule = build_directory_mods_recursive(directory.as_path())?;
            if !submodule.is_empty() {
                submodules.insert(Ident::new(dirname.as_str(), Span::call_site()), submodule);
            }
        } else if file_type.is_file() {
            let file_name = entry.file_name();
            let file_path = entry.path();
            if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
                continue;
            } else if file_path.extension() != Some(OsStr::new("rs")) {
                continue;
            }
            let file_ident = Ident::new(
                file_path
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    .as_str(),
                Span::call_site(),
            );
            files.push(file_ident);
        }
    }

    let dirnames = submodules.keys();
    let modstreams = submodules.values();
    if submodules.is_empty() && files.is_empty() {
        return Ok(TokenStream2::new());
    }
    Ok(quote! {
        #(
            pub mod #dirnames {
                #modstreams
            }
        )*
        #(
            pub mod #files;
            pub use self::#files::*;
        )*
    })
}
