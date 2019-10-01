impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i = num;
        while i > num / i {
            i = (i + num / i) / 2;
        }
        i * i == num
    }
}
