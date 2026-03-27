use quote::quote;
use proc_macro::TokenStream;
use std::path::{Path, PathBuf};
use syn::{LitBool, LitStr, Token, parse::{Parse, ParseStream}, parse_macro_input};



struct FsInput {
    path: LitStr,
    absolute: Option<LitBool>,
}

impl Parse for FsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let mut absolute = None;
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let ident: syn::Ident = input.parse()?;
            if ident == "absolute" {
                input.parse::<Token![=]>()?;
                absolute = Some(input.parse()?);
            } else {
                return Err(syn::Error::new_spanned(
                    ident, 
                    "unknown argument"
                ));
            }
        }
        Ok(FsInput { path, absolute })
    }
}





/// embed a dir from CARGO_MANIFEST_DIR as root or absolute path
/// 
/// bind_dir!(<path>, <false=CARGO_MANIFEST_DIR> or <true=absolute_path>);
/// 
/// usage: bind_dir!("from/manifest/dir", false);
/// usage: bind_dir!("/absolute/path", true);
#[proc_macro]
pub fn bind_dir(input: TokenStream) -> TokenStream {
    
    let macro_input = parse_macro_input!(input as FsInput);

    let dir_path = macro_input.path.value();

    let get_absolute_path = macro_input.absolute.map(|b| b.value).unwrap_or(false);

    let full_path = if get_absolute_path {
        PathBuf::from(&dir_path)
    } else {
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")).join(&dir_path)
    };
    
    if !full_path.exists() {
        return syn::Error::new_spanned(dir_path, format!("Directory '{}' not found.", full_path.display())).to_compile_error().into();
    }
    
    let mut files = Vec::new();
    let mut total_size = 0;

    let walker = walkdir::WalkDir::new(&full_path).into_iter().filter_map(|s| s.ok());
    
    for entry in walker {  
        let path = entry.path();
        if path.is_file() {
            let epoch = std::time::UNIX_EPOCH;
            let rel_path = path.strip_prefix(&full_path).unwrap();
            let rel_path_str = rel_path.to_string_lossy().replace('\\', "/");
            let path_str = path.to_string_lossy().to_string();
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            let (created, accessed, modified) = {
                let created = path.metadata().map(|m| m.created()).unwrap_or(Ok(std::time::SystemTime::now())).unwrap();
                let accessed = path.metadata().map(|m| m.accessed()).unwrap_or(Ok(std::time::SystemTime::now())).unwrap();
                let modified = path.metadata().map(|m| m.modified()).unwrap_or(Ok(std::time::SystemTime::now())).unwrap();
                (
                    created.duration_since(epoch).unwrap().as_secs(),
                    accessed.duration_since(epoch).unwrap().as_secs(),
                    modified.duration_since(epoch).unwrap().as_secs(),
                )
            };
            total_size += size;

            files.push(
                (
                    rel_path_str,
                    path_str,
                    size,
                    created,
                    accessed,
                    modified,
                    entry.depth()
                )
            );

        }
    }
    
    let vec_entries = files.iter().map(| (path, file_path, size, created, accessed, modified, depth) | {
        let filepath = LitStr::new(path, proc_macro2::Span::call_site());
        let path_to_file = LitStr::new(file_path, proc_macro2::Span::call_site());
        let size = *size;
        quote! {
            plugins_rs::FsFile {
                path: plugins_rs::EFsPath::Embed(#filepath),
                content: plugins_rs::EFsData::Embed(include_bytes!(#path_to_file)),
                size: #size,
                created: #created,
                accessed: #accessed,
                modified: #modified,
                depth: #depth,
            }
        }
    });
    
    let expanded = quote! {
        {
            plugins_rs::PluginsFs {
                total: #total_size,
                files: vec![
                    #(#vec_entries),*
                ],
            }
        }
    };
    
    TokenStream::from(expanded)
}