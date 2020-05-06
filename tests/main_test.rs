use std::path::PathBuf;
use indoc::indoc;
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


#[test]
fn create_spec_for_new_devkit() {
    let actual = plier::create_spec("", "java", vec!["bin".to_string()]);
    let expected = Ok(indoc!(r#"[java]
                                bins = ["bin"]
                                "#).to_string());

    assert_eq!(actual, expected);
}


#[test]
fn create_spec_when_others_exists() {
    let actual = plier::create_spec(
        "[java]\nbins = [\"bin\"]\n",
        "python",
        vec![".".to_string(), "Scripts".to_string()]);
    let expected = Ok(indoc!(r#"[java]
                                bins = ["bin"]

                                [python]
                                bins = [".", "Scripts"]
                                "#).to_string());

    assert_eq!(actual, expected);
}
