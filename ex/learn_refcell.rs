// 1、内部可变性：允许在使用不可变引用时改变数据
// 2、通过RefCell<T>在运行时检查借用规则（通常情况下，是在编译时检查借用规则），RefCell<T>代表其数据的唯一所有权

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    println!("a before: {:?}", a);
    println!("b before: {:?}", b);
    println!("c before: {:?}", c);
    *value.borrow_mut() += 10;
    println!("a after: {:?}", a);
    println!("b after: {:?}", b);
    println!("c after: {:?}", c);
    println!("End");
}
