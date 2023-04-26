pub struct FileStr {
    pub name: String,
    pub path: String,
    pub data: String,
}

pub use static_loader_macros::load_files_as_strs;
