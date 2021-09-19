use tempfile::tempdir;
use std::fs::File;
use std::io::{ Write};
use std::vec::Vec;
use std::path::Path;
use std::env;

use snippetlibrary::{SnippetCollection, Snippet};


#[test]
fn it_can_init_snippet(){
    let file_path: String = create_test_file();

    let snippet = Snippet::new(file_path.to_string());

    assert_eq!(file_path, snippet.path());
}

#[test]
fn it_can_get_title(){
    let file_path: String = create_test_file();

    let snippet = Snippet::new(file_path.to_string());

    assert_eq!("a-php-file.php", snippet.title());
}

#[test]
fn it_can_get_extension(){
    let file_path: String = create_test_file();

    let snippet = Snippet::new(file_path.to_string());

    assert_eq!("php", snippet.extension());
}

#[test]
fn it_can_get_content(){
    let path = env::current_dir().unwrap().join("tests/a-test-file.php");

    let snippet = Snippet::new(path.into_os_string().into_string().unwrap() );

    assert_eq!("<?php echo 'hello';?>", snippet.content());
}

fn create_test_file()->String{
    let dir = tempdir().unwrap();
    let path=  dir.path().to_str().unwrap().to_string();


    let file_path = dir.path().join("a-php-file.php");
    let mut file = File::create(file_path).expect("Problem");
    writeln!(file, "<?php echo 'hello';?>").expect("Could not write");

    dir.path().join("a-php-file.php").into_os_string().into_string().unwrap()
}