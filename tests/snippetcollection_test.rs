use tempfile::tempdir;
use std::fs::File;
use std::io::{ Write};
use std::vec::Vec;

use snippetlibrary::{SnippetCollection, Snippet};


#[test]
fn it_can_set_path(){
    let dir = tempdir().unwrap();
    let path=  dir.path().to_str().unwrap().to_string();

    let collection=  SnippetCollection::new(path);
    assert_eq!(dir.path().to_str().unwrap().to_string(), collection.path());
}
#[test]
fn it_can_load_snippets() {
    let dir = tempdir().unwrap();
    let path=  dir.path().to_str().unwrap().to_string();


    let file_path = dir.path().join("a-php-file.php");
    let mut file = File::create(file_path).expect("Problem");
    writeln!(file, "<?php echo 'hello';?>").expect("Could not write");

    let collection=  SnippetCollection::new(path);
    let snippets: Vec<Snippet> = collection.snippets();

    assert_eq!(1, snippets.len());
}