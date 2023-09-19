
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn Issublist<T:PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool
{
    let len1=_first_list.len();
    let len2=_second_list.len();
    if len1>len2 {return false;}
    if len1==0 {return true;}
    _second_list.windows(len1).any(|list| list==_first_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let issublist=Issublist(_first_list,_second_list);
    let issuperlist=Issublist(_second_list,_first_list);
    if issublist&&issuperlist {return Comparison::Equal;}
    if issublist&&!issuperlist {return  Comparison::Sublist;}
    if !issublist&&issuperlist {return Comparison::Superlist;}
    if !issublist&&!issuperlist {return Comparison::Unequal;}
    
    unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
