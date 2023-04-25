/*
fn call_print_str() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
在不删除已有代码的前提下，修改代码使得其能通过编译并输出。将修改方案放入文件print_str.rs中，并编写测试函数测试正确性。
 */

 pub fn call_print_str() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}