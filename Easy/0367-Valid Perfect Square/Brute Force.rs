impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i = 1;
        while i < num / i {
            i += 1;
        }
        i * i == num
    }
}
