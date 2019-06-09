/*
 * [可変な参照](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html#a%E5%8F%AF%E5%A4%89%E3%81%AA%E5%8F%82%E7%85%A7)
 * CreatedAt: 2019-06-09
 * &を「参照」といい、関数の引数に参照をとることを「借用」という。参照は変更不可。&mutで可変な参照になる。
 */
fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1); // &を先頭に付与して「参照」する（所有権をムーブしない） mutにして変更可能にする
    println!("{}", s1);
}
//fn change(s: &String)  {
fn change(s: &mut String)  {
    s.push_str(" world !!"); // error[E0596]: cannot borrow immutable borrowed content `*s` as mutable 借用は参照しかできない。変更不可。
}

