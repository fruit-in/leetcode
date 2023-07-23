impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        (num > 0) && (num & 0x55555555 == num) && (num & (num - 1) == 0)
    }
}
