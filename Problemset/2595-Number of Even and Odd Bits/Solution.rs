impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        vec![
            (n & 0x555).count_ones() as i32,
            (n & 0xaaa).count_ones() as i32,
        ]
    }
}
