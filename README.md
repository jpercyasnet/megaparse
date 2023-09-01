# megaparse
Rust-iced program to parse out from megacmd ls for comparison with rclone lsf 

Need to run the following terminal commands before using this app:
   rclone lsf --files-only -R --csv --format pst /pathofdirectory | sort > outputfile1
   
   mega-ls -l -r --time-format=ISO6081_WITH_TIME /cloudpathofdirectory > outputfile2

Run this app using outputfile2 for mega-ls and outputfile1 for rclone lsf.
   
This app will parse mega-ls and sort the file and execute Meld for comparison.
    
<img src="image/megap.png" width="800px" />
