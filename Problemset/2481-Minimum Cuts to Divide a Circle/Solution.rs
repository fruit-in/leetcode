impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 || n % 2 == 0 {
            n / 2
        } else {
            n
        }
    }
}
