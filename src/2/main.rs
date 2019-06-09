/*
 * [戻り値とスコープ](https://doc.rust-jp.rs/book/second-edition/ch04-01-what-is-ownership.html#a%E6%88%BB%E3%82%8A%E5%80%A4%E3%81%A8%E3%82%B9%E3%82%B3%E3%83%BC%E3%83%97)
 * CreatedAt: 2019-06-09
 */
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calc_length(s1);
    println!("{}, {}", s2, len);
}
fn calc_length(target: String) -> (String, usize) {
    let len = target.len();
    (target, len)
}

