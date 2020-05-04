use std::path::PathBuf;
use std::str::FromStr;
use plier;

#[test]
fn batch_filename_combines_executables_directory_with_given_filename() {
    let exe_path = PathBuf::from_str("path/to/binary.exe").unwrap();
    let actual = plier::batch_filename(exe_path, "batch.sh");

    assert_eq!(actual, PathBuf::from_str("path\\to\\batch.sh").unwrap());
}
