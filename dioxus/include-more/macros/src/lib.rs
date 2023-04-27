use ignore::WalkBuilder;
use quote::quote;

struct StaticLoaderInput {
    vis: Option<syn::Visibility>,
    name: syn::Ident,
    path: std::path::PathBuf,
}

// Build the StaticLoader from the parsed inputs
//
// ### Example input symbols
// ```
// pub static THEMES = {
//     path: "../assets/themes",
// };
// ```
impl syn::parse::Parse for StaticLoaderInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse the optional `pub` visibility
        let vis = input.parse::<syn::Visibility>().ok();

        // Parse the static designation
        input.parse::<syn::token::Static>()?;

        // Parse the name of the static variable
        let name = input.parse::<syn::Ident>()?;

        // Parse the equal sign separating the static variable from the fields
        input.parse::<syn::token::Eq>()?;

        // Parse fields out of braces
        let fields;
        syn::braced!(fields in input);
        let mut tmp_path: Option<syn::LitStr> = None;

        while !fields.is_empty() {
            // Parse the field name
            let k = fields.parse::<syn::Ident>()?;

            // Parse the colon separator
            fields.parse::<syn::Token![:]>()?;

            // Parse based on field name
            if k == "path" {
                tmp_path = Some(fields.parse()?);
            //} else if k == "locales" {
            //    locales_directory = Some(fields.parse()?);
            } else {
                return Err(syn::Error::new(k.span(), "Not a valid parameter"));
            }

            // No fields are left so stop looping
            if fields.is_empty() {
                break;
            }
            fields.parse::<syn::token::Comma>()?;
        }
        input.parse::<syn::token::Semi>()?;

        // Ensure the path is relative to the project root
        if let Some(_path) = tmp_path {
            let workspace_path = std::path::PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("./")),
            );
            let path = workspace_path.join(_path.value());
            if std::fs::metadata(&path).is_err() {
                return Err(syn::Error::new(_path.span(), &format!("Couldn't read path, this path should be relative to your crate's `Cargo.toml`. Looking for: {:?}", path)));
            }
            Ok(StaticLoaderInput { vis, name, path })
        } else {
            return Err(syn::Error::new(name.span(), "Missing required `path` field"));
        }
    }
}

// Read all the file paths recursively for the given path
fn get_file_paths<P: AsRef<std::path::Path>>(path: P) -> Vec<String> {
    let (tx, rx) = flume::unbounded();

    WalkBuilder::new(path).build_parallel().run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                if entry.file_type().as_ref().map_or(false, std::fs::FileType::is_file) {
                    tx.send(entry.path().display().to_string()).unwrap();
                }
            }

            ignore::WalkState::Continue
        })
    });

    // Collect the results and sort them
    let mut file_paths = rx.drain().collect::<Vec<_>>();
    file_paths.sort();
    file_paths
}

/// Loads all of your text files at compile time as `&'static str`s and creates a new static
/// variable that you can use in your app to access them. This allows you to easily ship
/// various content files as part of a single binary.
///
/// ### Example
/// ```no_compile
/// include_more::include_files_as_strs! {
///     // Declare the static access name
///     pub static THEMES = {
///         // Location of target files relative to project root
///         path: "../assets/themes",
///     };
/// }
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn include_files_as_strs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let StaticLoaderInput { vis, name, path } = syn::parse_macro_input!(input as StaticLoaderInput);

    // Now setup access to wrapping static-loader crate types
    let CRATE_NAME: proc_macro2::TokenStream = quote!(include_more);
    let LAZY: proc_macro2::TokenStream = quote!(#CRATE_NAME::once_cell::sync::Lazy);
    let FILE_AS_STR: proc_macro2::TokenStream = quote!(#CRATE_NAME::StaticFileStr);

    // Build the code template to load the static files into a vector
    let push_files = get_file_paths(path)
        .into_iter()
        .map(|file_path| quote!(files.push(#FILE_AS_STR::new(#file_path, include_str!(#file_path)));))
        .collect::<proc_macro2::TokenStream>();
    let files_quote = quote! {
        let mut files = Vec::<#FILE_AS_STR>::new();
        #push_files
        files
    };

    // Build the code template to load the static files vector into our access object
    let quote = quote! {
        #vis static #name : #LAZY<#CRATE_NAME::StaticLoaderStrs> = #LAZY::new(|| {
            static FILES_AS_STRS: #LAZY<Vec<#FILE_AS_STR>> = #LAZY::new(|| {
                #files_quote
            });
            #CRATE_NAME::StaticLoaderStrs::new(&FILES_AS_STRS)
        });
    };

    // println!("{}", quote);

    proc_macro::TokenStream::from(quote)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_file_paths() {
        let actual = get_file_paths("../tests/files");
        //println!("{:?}", actual);
        let expected = vec!["../tests/files/temp1", "../tests/files/temp2", "../tests/files/temp3"];
        assert_eq!(actual, expected);
    }
}
