// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 为 tests7 设置 TEST_FOO
    println!("cargo:TEST_FOO={}", timestamp);

    // 为 tests8 启用 pass 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

#[cfg(test)]
mod tests {
    // 不再需要导入 super::*，因为它未被使用

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
