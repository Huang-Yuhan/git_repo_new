/*fn clone_or_copy() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
修改代码，在不使用clone()的条件下使得其能通过编译并输出。将修改方案放入文件clone_or_copy.rs中，并编写测试函数测试正确性。
*/

pub fn clone_or_copy() {
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);
}