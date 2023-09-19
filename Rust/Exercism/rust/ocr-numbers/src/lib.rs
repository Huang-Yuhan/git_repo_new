// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}



pub fn parse_name(input:&str)->Result<String,Error>
{
    let number0 =" _ \n| |\n|_|\n   \n";
    let number1 ="   \n  |\n  |\n   \n";
    let number2 =" _ \n _|\n|_ \n   \n";    
    let number3 =" _ \n _|\n _|\n   \n";
    let number4 ="   \n|_|\n  |\n   \n";
    let number5 =" _ \n|_ \n _|\n   \n";
    let number6 =" _ \n|_ \n|_|\n   \n";
    let number7 =" _ \n  |\n  |\n   \n";
    let number8 =" _ \n|_|\n|_|\n   \n";
    let number9 =" _ \n|_|\n _|\n   \n";

    if input.cmp(number0) == std::cmp::Ordering::Equal
    {
        return Ok("0".to_string());
    }else if input.cmp(number1) == std::cmp::Ordering::Equal
    {
        return Ok("1".to_string());
    }else if input.cmp(number2) == std::cmp::Ordering::Equal
    {
        return Ok("2".to_string());
    }else if input.cmp(number3) == std::cmp::Ordering::Equal
    {
        return Ok("3".to_string());
    }else if input.cmp(number4) == std::cmp::Ordering::Equal
    {
        return Ok("4".to_string());
    }else if input.cmp(number5) == std::cmp::Ordering::Equal
    {
        return Ok("5".to_string());
    }else if input.cmp(number6) == std::cmp::Ordering::Equal
    {
        return Ok("6".to_string());
    }else if input.cmp(number7) == std::cmp::Ordering::Equal
    {
        return Ok("7".to_string());
    }else if input.cmp(number8) == std::cmp::Ordering::Equal
    {
        return Ok("8".to_string());
    }else if input.cmp(number9) == std::cmp::Ordering::Equal
    {
        return Ok("9".to_string());
    }else{
        return Ok("?".to_string());
    }

}

pub fn convert(input: &str) -> Result<String, Error> {
    //变成字符串数组
    let input_str:Vec<&str> = input.split("\n").collect();
    //判断有几行文字
    if input_str.len()%4 !=0 {return Err(Error::InvalidRowCount(input_str.len()));}
    let row = input_str.len()/4;
    //判断每行文字的长度
    if input_str[0].len()%3 !=0 {return Err(Error::InvalidColumnCount(input_str[0].len()));}
    let column = input_str[0].len()/3;
    let mut result = String::new();
    for i in 0..row
    {
        for j in 0..column
        {
            let mut temp = String::new();
            for k in 0..4
            {
                temp.push_str(&input_str[i*4+k][j*3..j*3+3]);
                temp.push_str("\n");
            }
            result.push_str(&parse_name(&temp)?);
        }
        if i!=row-1{ result.push_str(",");}
    }
    Ok(result)
}
