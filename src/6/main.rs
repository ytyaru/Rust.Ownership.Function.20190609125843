/*
 * [可変な参照](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html#a%E5%8F%AF%E5%A4%89%E3%81%AA%E5%8F%82%E7%85%A7)
 * CreatedAt: 2019-06-09
 * 可変な参照は1つだけ。他の言語にはない大きな制約。
 * 不変な参照をしているときは可変な参照ができない！
 */
fn main() {
    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &mut s1; // error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
}

