impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() - 1 {
            i += 1 + bits[i] as usize;
        }
 
        i == bits.len() - 1
    }
}
