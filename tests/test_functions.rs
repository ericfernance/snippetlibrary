use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec::Vec;

use tempfile::{tempdir, TempDir};

lazy_static! {
        static ref DIR:TempDir = tempdir().unwrap();
    }

pub fn create_test_file() -> String {
    //let dir = tempdir().unwrap();
    let path = DIR.path().to_str().unwrap().to_string();
    //let path=  dir.path().to_str().unwrap().to_string();


    let file_path = DIR.path().join("a-php-file.php");
    //let file_path = dir.path().join("a-php-file.php");
    let mut file = File::create(file_path).expect("Problem");
    writeln!(file, "<?php echo 'hello';?>").expect("Could not write");

    DIR.path().join("a-php-file.php").into_os_string().into_string().unwrap()
    //dir.path().join("a-php-file.php").into_os_string().into_string().unwrap()
}

