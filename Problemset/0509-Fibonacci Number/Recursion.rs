impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            n
        } else {
            Self::fib(n - 1) + Self::fib(n - 2)
        }
    }
}
