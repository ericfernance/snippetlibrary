use snippetlibrary::SnippetCollection;

fn main() {
    println!("Hello, world!");
    //let collection = SnippetCollection::new("test");

    //collection.you_suck();
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sanity(){
        assert_eq!(1,1);
    }
}