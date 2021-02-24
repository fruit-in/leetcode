use std::iter;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        if k == 26 * n {
            "z".repeat(n as usize)
        } else {
            iter::repeat('a')
                .take((n - (k - n) / 25 - 1) as usize)
                .chain(iter::once((97 + (k - n) % 25) as u8 as char))
                .chain(iter::repeat('z').take(((k - n) / 25) as usize))
                .collect()
        }
    }
}
