// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 在宏定义中，每个规则后面都要有分号，该宏才能被正确解析。
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();

    // Rust 宏按顺序匹配模式。因此先匹配无参数的 ()，再匹配带表达式的 ($val:expr)。
    // 如果顺序颠倒，带参数的模式可能会覆盖无参数的模式（虽然在这个例子中不会发生）。
    my_macro!(7777);

}
