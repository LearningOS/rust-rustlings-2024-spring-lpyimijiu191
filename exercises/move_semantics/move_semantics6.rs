// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone()); // 1.如果只用data，那么get_char()的执行周期结束后，data会被释放掉。因此用data.clone()。

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // 2.这里用&data.to_uppercase()会报错，因此就不用指针了。

    println!("{}", data);
}
