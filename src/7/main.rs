/*
 * [宙に浮いた参照](https://doc.rust-jp.rs/book/second-edition/ch04-02-references-and-borrowing.html#a%E5%AE%99%E3%81%AB%E6%B5%AE%E3%81%84%E3%81%9F%E5%8F%82%E7%85%A7)
 * CreatedAt: 2019-06-09
 * 可変な参照は1つだけ。他の言語にはない大きな制約。
 * 不変な参照をしているときは可変な参照ができない！
 */
fn main() {
    let s = dangle();
}
fn dangle() -> &String { // dangle関数はString型の参照を返す　error[E0106]: missing lifetime specifier
    let s = String::from("hello"); // sでヒープ領域を確保
    &s // 参照を返す
} // sを解放する（解放済みの領域を参照することになり危険。だがRustならコンパイルエラーになる）
