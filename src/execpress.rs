use std::path::Path;

pub fn execpress (mega_value: String, rclone_value: String, rows_num: u64, utc_value: String) -> (u32, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "all good and now process execution".to_string();
     if Path::new(&rclone_value).exists() {
         if Path::new(&mega_value).exists() {
             if rows_num > 10 {
                 let utc_int: i32 = utc_value.parse().unwrap_or(-9999);
                 if utc_int == -9999 {
                    errcode = 1;
                    errstring = "UTC value is not numeric".to_string();
                 } else {
                    if utc_int > 23 || utc_int < -23 {
                        errcode = 2;
                        errstring = "UTC value is either greater than 23 or less than -23".to_string();
                    }
                 }
             } else {
                 errcode = 3;
                 errstring = "The number of rows is less than 11".to_string();
             }
         } else {
             errstring = "the mega ls file does not exist".to_string();
             errcode = 4;
         }
     } else {
         errstring = "the rclone file does not exist".to_string();
         errcode = 5;
     }
     (errcode, errstring)
}

