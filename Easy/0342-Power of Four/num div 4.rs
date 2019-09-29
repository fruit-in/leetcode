impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let mut num = num;
        if num <= 0 {
            false
        } else {
            while num % 4 == 0 {
                num /= 4;
            }
            num == 1
        }
    }
}
