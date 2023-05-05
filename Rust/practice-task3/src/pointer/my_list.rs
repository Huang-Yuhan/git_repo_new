use std::ops::Index;

pub mod my_list
{
    use List::*;
   pub enum List {
       Cons(i32,Box<List>),
       Nil
   }

   impl List {
       pub fn new()->List{
        Nil
       }

       pub fn push(self,data:i32)->List{
        Cons(data, Box::new(self))
       }

       pub fn len(&self) -> u32{
        match *self {
            Cons(_,ref tail)=> 1+tail.len(),
            Nil=> 0
        }
       }

       pub fn pop(self)->List{
        match self {
            Nil=> Nil,
            Cons(_,tail)=> *tail
        }
       }

       pub fn at(&self,index:u32)->&i32{
        let mut tmp=self;
        for i in 0..index{
            tmp=match *tmp {
                Nil=> panic!("at error"),
                Cons(_,ref tail )=> tail
            }
        }
        match tmp {
            Nil=> panic!("at error"),
            Cons(val,_)=>val
        }
       }
   }

   
}

impl Index<u32> for my_list::List {
    fn index(&self, index: u32) -> &Self::Output {
        self.at(index)
    }

    type Output=i32;

}