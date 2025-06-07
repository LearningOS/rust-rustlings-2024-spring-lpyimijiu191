// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);

    // 将for循环替换为if let Some(x) = option的方式。不仅更符合Rust的惯用写法，也能通过Clippy的检查。
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
