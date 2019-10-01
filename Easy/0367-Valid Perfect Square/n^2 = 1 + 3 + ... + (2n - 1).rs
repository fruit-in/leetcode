impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut num = num;
        let mut i = 1;
        while num > 0 {
            num -= i;
            i += 2;
        }
        num == 0
    }
}
