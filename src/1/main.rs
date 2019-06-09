/*
 * [戻り値とスコープ](https://doc.rust-jp.rs/book/second-edition/ch04-01-what-is-ownership.html#a%E6%88%BB%E3%82%8A%E5%80%A4%E3%81%A8%E3%82%B9%E3%82%B3%E3%83%BC%E3%83%97)
 * CreatedAt: 2019-06-09
 */
fn main() {
    let s1 = create_heap_string(); // 戻り値は所有権がムーブする
    println!("{}", s1);
    let s2 = take_and_give(s1); // 戻り値は所有権がムーブする
    println!("{}", s2);
}
fn create_heap_string() -> String {
    let msg = String::from("hello");
    msg
}
fn take_and_give(msg: String) -> String {
    msg
}

