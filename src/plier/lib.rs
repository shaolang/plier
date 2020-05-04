use std::path::PathBuf;
use std::str::FromStr;

pub fn batch_filename(exe_path: PathBuf, filename: &str) -> PathBuf {
    let exe_str = exe_path.to_str().expect("Unable to get binary's path");
    let mut file = PathBuf::from_str(exe_str).unwrap().parent()
        .expect("Unable to get binary's directory").to_owned();
    file.push(filename);
    file
}
