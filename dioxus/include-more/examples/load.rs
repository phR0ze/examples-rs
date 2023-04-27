use include_more;

include_more::include_files_as_strs! {
    static FILES = {
        path: "tests/files",
    };
}

fn main() {
    let expected = vec![
        "/builtin/tests/files/temp1",
        "/builtin/tests/files/temp2",
        "/builtin/tests/files/temp3",
    ];
    assert_eq!(FILES.iter().map(|x| x.path.clone()).collect::<Vec<_>>(), expected);
    //println!("{:?}", FILES.files[0]);
}
