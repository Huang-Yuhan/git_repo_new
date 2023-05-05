# Box\<T\>

e.g.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

```

### Box可以用来创建递归类型

例如C++里面的listNode

```cpp
struct listNode
{
    listNode* next;
    int val;
};
```

但是rust对于这种类型只能使用Box创建

因为rust需要创建的时候这个数据类型大小是多少

[官方的链表实现](https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum/testcase_linked_list.html)