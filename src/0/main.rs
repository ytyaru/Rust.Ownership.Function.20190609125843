/*
 * [所有権と関数](https://doc.rust-jp.rs/book/second-edition/ch04-01-what-is-ownership.html#a%E6%89%80%E6%9C%89%E6%A8%A9%E3%81%A8%E9%96%A2%E6%95%B0)。
 * CreatedAt: 2019-06-09
 */
fn main() {
    let s = String::from("hello");
    show(s); // 所有権がmain関数のsからshow関数のmsgへムーブした
//    println!("{}", s); // error[E0382]: use of moved value: `s`
}
fn show(msg: String) {
    println!("{}", msg);
}
