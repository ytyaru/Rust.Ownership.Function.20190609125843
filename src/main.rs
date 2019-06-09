/*
 * 所有権と関数。
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
