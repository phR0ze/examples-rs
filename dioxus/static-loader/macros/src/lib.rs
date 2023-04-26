use ignore::WalkBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, LitStr, Result, Token,
};

struct StaticLoader {
    vis: Option<syn::Visibility>,
    name: Ident,
    path: PathBuf,
}

// Build the StaticLoader from the parsed inputs
// ```
// static THEMES = {
//     // The path to load the files from. This should be relative to the root fo the project
//     path: "../assets/themes",
// };
// ```
impl Parse for StaticLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the optional `pub` visibility
        let vis = input.parse::<syn::Visibility>().ok();

        // Parse the static designation
        input.parse::<token::Static>()?;

        // Parse the name of the static variable
        let name = input.parse::<Ident>()?;

        // Parse the equal sign separating the static variable from the fields
        input.parse::<token::Eq>()?;

        // Parse fields out of braces
        let fields;
        braced!(fields in input);
        let mut tmp_path: syn::LitStr;

        while !fields.is_empty() {
            // Parse the field name
            let k = fields.parse::<Ident>()?;

            // Parse the colon separator
            fields.parse::<syn::Token![:]>()?;

            // Parse based on field name
            if k == "path" {
                tmp_path = fields.parse()?;
            //} else if k == "locales" {
            //    locales_directory = Some(fields.parse()?);
            } else {
                return Err(syn::Error::new(k.span(), "Not a valid parameter"));
            }

            if fields.is_empty() {
                break;
            }
            fields.parse::<token::Comma>()?;
        }
        input.parse::<token::Semi>()?;

        // Ensure the path is relative to the project root
        let workspace_path =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("./")));
        let path = workspace_path.join(tmp_path.value());

        if std::fs::metadata(&path).is_err() {
            return Err(syn::Error::new(tmp_path.span(), &format!("Couldn't read path, this path should be relative to your crate's `Cargo.toml`. Looking for: {:?}", path)));
        }

        Ok(StaticLoader { vis, name, path })
    }
}

// Read all the file paths recursively for the given path
fn get_file_paths<P: AsRef<Path>>(path: P) -> Vec<String> {
    let (tx, rx) = flume::unbounded();

    WalkBuilder::new(path).build_parallel().run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                if entry.file_type().as_ref().map_or(false, fs::FileType::is_file) {
                    tx.send(entry.path().display().to_string()).unwrap();
                }
            }

            ignore::WalkState::Continue
        })
    });

    rx.drain().collect::<Vec<_>>()
}

/// Loads all of your text file resources at compile time as `&'static str`s and
/// and creates a new `StaticLoader` static variable that you can use in your
/// program to access them. This allows you to easily ship various content files
/// as part of a single binary.
///
/// ### Example
/// ```no_compile
/// static_loader::load_files_as_strs! {
///     // Declare our `StaticLoader` named `THEMES`.
///     static THEMES = {
///         // The path to load the files from. This should be relative to the root fo the project
///         path: "../assets/themes",
///     };
/// }
/// ```
#[proc_macro]
pub fn load_files_as_strs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let StaticLoader { vis, name, path } = parse_macro_input!(input as StaticLoader);

    let CRATE_NAME: TokenStream = quote!(static_loader);
    let FILE_STR: TokenStream = quote!(#CRATE_NAME::FileStr);
    let HASHMAP: TokenStream = quote!(std::collections::HashMap);

    //let data: Vec<u8> = std::fs::read(&filepath).expect(&format!("File {:?} could not be read", filepath));
    //     let len = data.len();
    //     TokenStream::from(quote! {
    //         #[link_section = ".progmem.data"]
    //         static #name: [u8; #len] = [#(#data),*];
    //     })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_file_paths() {
        let files = get_file_paths(".");
        println!("{:?}", files);
        assert!(files.iter().find(|x| x == &"./src/lib.rs").is_some());
    }
}
