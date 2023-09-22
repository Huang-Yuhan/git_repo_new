pub mod ownership;
pub mod pointer;
pub mod cli;
#[cfg(test)]
mod test_ownership {
    use crate::ownership::clone_or_copy::clone_or_copy;
    use crate::ownership::i_want_both::i_want_both_1;
    use crate::ownership::i_want_both::i_want_both_2;
    use crate::ownership::i_want_both::i_want_both_3;
    use crate::ownership::print_str::call_print_str;

    
    #[test]
    fn test_ownership() {
        i_want_both_1();
        i_want_both_2();
        i_want_both_3();
    }

    #[test]
    fn test_print_str()
    {
        call_print_str();
    }

    #[test]
    fn test_clone_or_copy()
    {
        clone_or_copy();
    }
}

mod test_pointer{

    use crate::pointer::my_list::my_list::List;

    #[test]
    fn test_my_list()
    {
        let mut lis=List::new();
        lis=lis.push(5);
        lis=lis.push(4);
        lis=lis.push(1);
        assert_eq!(lis[0],1);
        assert_eq!(lis[1],4);
        assert_eq!(lis[2],5);
        lis=lis.pop();
        assert_eq!(lis[0],4);
        lis=lis.pop();
        assert_eq!(lis[0],5);
    }
}

mod test_two_boxes{
    use crate::pointer::twoBoxes::two_boxes;
    #[test]
    fn test_two_boxes(){
        let (sum,first,second) = two_boxes::two_boxes_func();
        assert_eq!(sum,3);
        assert_eq!(first,1);
        assert_eq!(second,2);
    }
}
