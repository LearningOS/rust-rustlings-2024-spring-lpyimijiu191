// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    // ()：这是一个无参数的宏。
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 1.所有宏调用必须以“!”结尾。；2.传递的参数必须为空。
    my_macro!();
}
