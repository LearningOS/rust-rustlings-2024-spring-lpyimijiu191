// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    // Rust中定义在模块內的宏默认是私有的，不能被模块外部访问。
    // 因此，使用 #[macro_export] 属性将宏导出到根作用域。如此一来，即使宏定义在模块内，也可以在外部直接被调用。
    #[macro_export]
    
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
