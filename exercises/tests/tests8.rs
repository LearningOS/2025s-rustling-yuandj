// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// tests8.rs 文件内容：
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        // 当没有 pass 特性时会编译以下代码
        panic!("This line should not compile when 'pass' feature is enabled");
    }
}