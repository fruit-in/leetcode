impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = bits.len() - 1;
        while i > 0 && bits[i - 1] == 1 {
            i -= 1;
        }

        (bits.len() - 1 - i) % 2 == 0
    }
}
