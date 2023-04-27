//! # Include More: Easily include multiple files statically in your binary or library
//!
//! `include-more` lets you easily include files at compile time as `&'static str`s and
//! creats a static variable to access them. This allows you to easily ship various
//! content files as part of a single binary or library.
//!
//! ### Pathing
//! Any files included at compile time will need to be specified with the `path` field
//! relative to the project root. The resulting file object will have a path starting
//! with `/builtin` as a substitute for the project root path as an indicator from
//! where it came from.
//!
//! ### Examples
//! ```
//! use include_more;
//!
//! include_more::include_files_as_strs! {
//!     static FILES = {
//!         path: "tests/files",
//!     };
//! }
//!
//! let expected = vec!["/builtin/tests/files/temp1", "/builtin/tests/files/temp2"];
//! assert_eq!(FILES.iter().map(|x| x.path.clone()).collect::<Vec<_>>(), expected);
//! ```
use std::slice::Iter;

// Re-exports for macro crate
pub use once_cell;

/// StaticLoader is loaded with static data at compile time via the
/// [`load_files_as_strs`] proc_macro.
pub struct StaticLoaderStrs {
    #[allow(dead_code)]
    files: &'static Vec<StaticFileStr>,
}

impl StaticLoaderStrs {
    /// Construct a new `StaticLoaderStrs` instance.
    ///
    /// This is exposed publicly so that it can be used inside the
    /// macros, but is not meant to be called directly.
    #[doc(hidden)]
    pub fn new(files: &'static Vec<StaticFileStr>) -> Self {
        Self { files }
    }

    /// Iterate over the static file entries
    ///
    /// ### Examples
    /// ```
    /// use include_more;
    ///
    /// include_more::include_files_as_strs! {
    ///     static FILES = {
    ///         path: "tests/files",
    ///     };
    /// }
    ///
    /// let expected = vec!["/builtin/tests/files/temp1", "/builtin/tests/files/temp2", "/builtin/tests/files/temp3"];
    /// assert_eq!(FILES.iter().map(|x| x.path.clone()).collect::<Vec<_>>(), expected);
    /// ```
    pub fn iter(&self) -> Iter<'_, StaticFileStr> {
        self.files.iter()
    }
}

/// StaticFileStr provides a public structure to store the compile-time file data as a String
///
/// ### Examples
/// ```
/// use include_more;
///
/// include_more::include_files_as_strs! {
///     static FILES = {
///         path: "tests/files",
///     };
/// }
///
/// let expected = vec!["/builtin/tests/files/temp1", "/builtin/tests/files/temp2", "/builtin/tests/files/temp3"];
/// assert_eq!(FILES.iter().map(|x| x.path.clone()).collect::<Vec<_>>(), expected);
/// ```
#[derive(Debug)]
pub struct StaticFileStr {
    pub path: String,
    pub data: String,
}

impl StaticFileStr {
    /// Used to convert the static include string into a StaticFileStr object
    #[doc(hidden)]
    pub fn new(path: &str, data: &str) -> Self {
        let workspace_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from("./"));
        Self {
            path: format!("/builtin{}", path.trim_start_matches(&workspace_path).to_string()),
            data: data.to_owned(),
        }
    }
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
pub use include_more_macros::include_files_as_strs;
