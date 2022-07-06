impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut count_s = [0; 26];
        let mut count_t = [0; 26];

        s.bytes().for_each(|c| count_s[(c - b'a') as usize] += 1);
        target
            .bytes()
            .for_each(|c| count_t[(c - b'a') as usize] += 1);

        (0..26)
            .filter(|&i| count_t[i] > 0)
            .map(|i| count_s[i] / count_t[i])
            .min()
            .unwrap()
    }
}
