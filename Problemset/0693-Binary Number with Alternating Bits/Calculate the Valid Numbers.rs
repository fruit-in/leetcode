impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut i = 1;
        while i > 0 && i < n {
            match i % 2 {
                1 => i = 2 * i,
                _ => i = 2 * i + 1,
            };
        }

        i == n
    }
}
