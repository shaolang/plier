use std::path::PathBuf;
use plier;

#[test]
fn batch_filename_combines_executables_directory_with_given_filename() {
    let exe_path = PathBuf::from("path/to/binary.exe");
    let actual = plier::batch_filename(exe_path, "batch.sh");
    let mut expected = PathBuf::from("path");
    expected.push("to");
    expected.push("batch.sh");

    assert_eq!(actual, expected);
}
