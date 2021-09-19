use std::fs::{read_dir };
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Default, Debug)]
pub struct Snippet{
    path: String,
    content: String
}

impl Snippet {
    pub fn new(path: String) -> Snippet{
        Snippet {
            path: path,
            ..Default::default()
        }
    }

    pub fn path(&self)->&str{
        self.path.as_str()
    }

    pub fn title(&self)->&str {
        self.path.split("/").into_iter().last().unwrap()
    }

    pub fn extension(&self) -> &str {
        self.title().split(".").into_iter().last().unwrap()
    }

    pub fn content(&self) -> String {
        let mut file = File::open(Path::new(self.path())).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents.to_string()
    }
}

pub struct SnippetCollection {
    path: String,
}

impl SnippetCollection {

    pub fn new(path: String) -> SnippetCollection{
        SnippetCollection {
            path: path,
        }
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn snippets(&self) -> Vec<Snippet>{
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

