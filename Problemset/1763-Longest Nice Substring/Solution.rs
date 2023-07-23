impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let s = s.as_bytes();
        let mut ret = "";

        for i in 0..s.len() {
            let mut counter = [0; 26];

            for j in i..s.len() {
                match s[j] {
                    b'A'..=b'Z' => counter[(s[j] - b'A') as usize] |= 1,
                    _ => counter[(s[j] - b'a') as usize] |= 2,
                }
                if counter.iter().all(|&c| c % 3 == 0) && j - i + 1 > ret.len() {
                    ret = std::str::from_utf8(&s[i..=j]).unwrap();
                }
            }
        }

        ret.to_string()
    }
}
