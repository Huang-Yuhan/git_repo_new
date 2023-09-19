
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum=0;
    let mut i=1;
    for code_digit in code.chars().rev()
    {
        if code_digit==' ' {continue;}
        if code_digit.is_numeric()==false {return false;}
        let digit=code_digit.to_digit(10);
        match digit {
            Some(value) => 
            {
                if i%2==0 
                {
                    sum+=if value*2>9 {value*2-9}else{value*2};
                }
                else 
                {
                    sum+=value;
                }
            }
            None => return false
        }
        i+=1;
    }
    i>2&&sum%10==0
}
