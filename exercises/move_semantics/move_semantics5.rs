// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.


// 在任何给定时间，要么只能有一个可变引用，要么可以有多个不可变引用。

// 不可变引用和可变引用之间是互斥的，不能同时存在。
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    
    *z += 1000;
    assert_eq!(x, 1200);
}
