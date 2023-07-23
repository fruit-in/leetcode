impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = [0; 10];

        for d in num.bytes() {
            count[(d - b'0') as usize] += 1;
        }

        num.bytes()
            .enumerate()
            .all(|(i, d)| count[i] == (d - b'0') as usize)
    }
}
