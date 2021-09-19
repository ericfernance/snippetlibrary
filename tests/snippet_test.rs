use tempfile::{tempdir, TempDir};
use std::fs::File;
use std::io::{ Write};
use std::vec::Vec;
use std::path::Path;
use std::env;

mod test_functions;



#[macro_use]
extern crate lazy_static;

use snippetlibrary::{SnippetCollection, Snippet};

lazy_static!{
    static ref DIR:TempDir = tempdir().unwrap();
}

#[test]
fn it_can_init_snippet(){
    let file_path: String =  test_functions::create_test_file();

    let snippet = Snippet::new(file_path.to_string());

    assert_eq!(file_path, snippet.path());
}

#[test]
fn it_can_get_title(){
    let file_path: String =  test_functions::create_test_file();

    let snippet = Snippet::new(file_path.to_string());

    assert_eq!("a-php-file.php", snippet.title());
}

#[test]
fn it_can_get_extension(){
    let file_path: String =  test_functions::create_test_file();

    let snippet = Snippet::new(file_path.to_string());

    assert_eq!("php", snippet.extension());
}

#[test]
fn it_can_get_content(){
    let file_path: String =  test_functions::create_test_file();

    //let path = env::current_dir().unwrap().join("tests/a-test-file.php");

    //let snippet = Snippet::new(path.into_os_string().into_string().unwrap() );
    let snippet = Snippet::new(file_path.to_string() );

    assert_eq!("<?php echo 'hello';?>\n", snippet.content());
}

