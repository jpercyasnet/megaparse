use native_dialog::FileDialog;
use std::path::{Path, PathBuf};
pub fn inputpress (inputval: String) -> (u32, String, String) {
     let errcode: u32;
     let errstring: String;
     let mut new_input: String;
     if Path::new(&inputval).exists() {
         let getpath = PathBuf::from(&inputval);
         let getdir = getpath.parent().unwrap();
         new_input = getdir.to_str().unwrap().to_string();
     } else {
         new_input = "/".to_string();
     }
     let newfile = FileDialog::new()
        .set_location(&new_input)
        .show_open_single_file()
        .unwrap();
     if newfile == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_input = newfile.as_ref().expect("REASON").display().to_string();
         errstring = "got file".to_string();
         errcode = 0;
     } 
    (errcode, errstring, new_input)
}

