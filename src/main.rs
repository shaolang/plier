use std::env;
use plier;

fn main() {
    let fname = plier::batch_filename(env::current_exe().unwrap(), "batch.bat");
    println!("{}", fname.to_str().unwrap());
}
