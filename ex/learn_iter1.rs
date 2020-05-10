//　１、迭代器负责遍历序列中的每一项和决定何时结束的逻辑
//　２、创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
//　３、每个迭代器都实现了iterator trait，iterator trait定义在标准库中：
// trait iterator {
//     type Item;
//     fn next(mut self) -> Option<Self::Item>;
//     // type Item和Self::Item这种用法叫做定义trait的关联类型
// }

// next是Iterator被要求实现的唯一的一个方法，next一次返回一个元素，当迭代器结束时候，返回None

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // 到目前为止，不会对v1产生任何影响
                                 // for val in v1_iter {
                                 // println!("val = {}", val);
                                 // }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v); // 1
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v); // 2
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v); // 3
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v); // 4
    } else {
        println!("At end");
    }
    // ---------------迭代可变使用-----------------
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();

    println!("End");
}
