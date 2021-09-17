use std::fs::{read_dir};
use std::path::Path;

#[derive(Default)]
pub struct Snippet{
    path: String,
    title: String,
    content: String
}

impl Snippet {
    pub fn new(path: String) -> Snippet{
        Snippet {
            path: path,
            ..Default::default()
        }
    }
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
        let mut snippets: Vec<Snippet> = vec![];
        let dir_path = Path::new(self.path.as_str());
        for entry in read_dir(dir_path).unwrap(){
            let entry_path = entry.unwrap().path();
            snippets.push(Snippet::new(entry_path.into_os_string().into_string().unwrap()));
        }
        snippets
    }

    pub fn add_snippet(&self,snippet: Snippet){}
}

