// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // y在第三行被使用后，其生命周期结束，第四行声明z时，x的可变引用已经被释放，所以可以再次获取。这样就能通过编译。
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
