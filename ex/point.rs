/**
 * 1、最简单最直接的智能指针是box，其类型为Box<T>。
 * box允许将值放在堆上而不是栈上，留着栈上的则是指向堆数据的指针。除了数据被存储在堆上外，box没有任何性能损失。
 * 2、box适用于如下场景：
 * （1）当有一个在编译时未知大小的类型，而又需要确切的上下文中使用这个类型值的时候；
 * （举例子：在一个list环境下，存放数据，但是每个元素的大小在编译时又不确定）
 * （2）当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候；
 * （3）当希望拥有一个值只关心它的类型是否实现了特定trait而不是其具体雷兴时。
 * */
// #[derive(Debug)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use List::{Cons, Nil};
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("end");
}
