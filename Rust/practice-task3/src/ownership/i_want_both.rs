/*fn i_want_both() {
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}*/

/*
提出至少三种修改思路，在不更改第4行的前提下，使该段代码能够通过编译并输出。
创建文件i_want_both.rs，
将三种修改方案分别命名为i_want_both_1()，i_want_both_2(),i_want_both_3()，
并编写测试函数测试正确性。
*/


pub fn i_want_both_1()
{
    //使用clone()
    let x = String::from("hello, world");
    let y=x.clone();
    assert_eq!(x,y);
}

pub fn i_want_both_2()
{
    //使用slice()
    let x=String::from("hello, world");
    let y=&x[..];
    assert_eq!(x,y);
}

pub fn i_want_both_3()
{
    //引用
    let x=String::from("hello, world");
    let y=&x;
    assert_eq!(x,*y);
}