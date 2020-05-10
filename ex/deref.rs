// 实现Deref trait允许我们重载解引用运算符
fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = Box::new(x); // 数据存在堆上
    assert_eq!(5, *z);
    println!("End");
}
