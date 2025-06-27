impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as u8;
        let mut dp = [0; 26];

        for c in s.bytes() {
            dp[(c - b'a') as usize] = (b'a'.max(c - k)..=b'z'.min(c + k))
                .map(|i| dp[(i - b'a') as usize])
                .max()
                .unwrap()
                + 1;
        }

        *dp.iter().max().unwrap()
    }
}
