/*
 * [参照と借用](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html)
 * CreatedAt: 2019-06-09
 * &を「参照」といい、関数の引数に参照をとることを「借用」という。
 */
fn main() {
    let s1 = String::from("hello");
    let len = calc_length(&s1); // &を先頭に付与して「参照」する（所有権をムーブしない）
    println!("{}, {}", s1, len); // &で参照したから戻り値にして所有権をムーブせずとも参照できる
}
fn calc_length(s: &String) -> usize {
    s.len()
} // sは参照のため所有権がムーブされずスコープはmain関数のままなので本関数の末尾でメモリ解放されない

