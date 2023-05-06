pub trait SearchWord
{
    fn get_count(&self,word:&str)->usize;
    fn add_content(&mut self,content:&String);
}

pub struct WordSearch
{
    file:String
}

impl SearchWord for WordSearch{
    fn add_content(&mut self,content:&String) {
        self.file=content.clone();
    }
    fn get_count(&self,word:&str)->usize {
        let ans:Vec<&str>=self.file.matches(word).collect();
        ans.len()

    }
}

impl WordSearch {
    pub fn new()->WordSearch{
        WordSearch { file: String::from("") }
    }
}