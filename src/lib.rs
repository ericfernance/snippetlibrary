use std::fs::{read_dir};
use std::path::Path;

pub struct Snippet{

}

pub struct SnippetCollection {
    path: String,
}

impl SnippetCollection {
    pub fn you_suck(&self) {
        println!("you suck");
    }

    pub fn new(path: String) -> SnippetCollection{
        SnippetCollection {
            path: path,
        }
    }

    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }

    pub fn get_snippets(&self) -> Vec<Snippet>{
        let dir_path = Path::new(self.path.as_str());
        for entry in read_dir(dir_path).unwrap(){
            let entry_path = entry.unwrap().path();
            println!("************************* {}", entry_path.into_os_string().into_string().unwrap())
        }
        vec![]
    }
}

