/// Generate a string id based on the current file, line, column concatenation
/// * ***WARNING*** only unique if you call it in a unique code location
#[macro_export]
macro_rules! id {
    () => {{
        concat!(file!(), ":", line!(), ":", column!())
    }};
}
