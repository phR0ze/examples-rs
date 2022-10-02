/// Generate a string id based on the current file, line, column concatenation
#[macro_export]
macro_rules! gid {
    () => {{
        concat!(file!(), line!(), column!())
    }};
}
