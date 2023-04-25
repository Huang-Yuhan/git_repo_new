pub mod ownership;
#[cfg(test)]
mod test {
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
