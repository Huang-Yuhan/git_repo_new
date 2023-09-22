
/*
  1. 画出内存中arr 、first、second和sum的关系图，并标明哪些变量处在堆上，哪些处在栈上。
  2. 在two_boxes函数运行结束后，上述变量又分别处在哪些位置，彼此的关系是什么？
  3. 完善这个函数，在不修改2至4行的前提下，使其返回first,second和sum的值。
*/

pub mod two_boxes
{
  pub fn two_boxes_func()->(i32,i32,i32){
    let arr = vec![Box::new(1),Box::new(2)];
    let (first,second) = (&arr[0],&arr[1]);
    let sum = **first + **second;
    (sum,**first,**second)
  }
}


