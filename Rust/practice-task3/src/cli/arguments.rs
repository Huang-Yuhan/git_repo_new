
pub trait ParseArg{
    fn parse(&mut self) -> Config;
    fn add_argument(&mut self, arg: String) -> Result<(), AddArgumentErr>;
}

pub struct Config
{
    pub input_file:String,
    pub word:String,
    pub output_file:String,
}

pub enum AddArgumentErr {
    EmptyArg
}

pub struct ArgParser 
{
    args:Vec<String>
}

impl ArgParser {
    pub fn new()-> ArgParser
    {
        ArgParser { args: Vec::new() }
    }
}

impl ParseArg for ArgParser {
    fn parse(&mut self) -> Config {
        Config { input_file: self.args[0].clone(), word: self.args[1].clone(), output_file: self.args[2].clone() }
    }
    fn add_argument(&mut self, arg: String) -> Result<(), AddArgumentErr> {
        if arg.is_empty(){
            return Result::Err(AddArgumentErr::EmptyArg)
        }
        else {
            self.args.push(arg);
            return Result::Ok(())
        }
    }
}