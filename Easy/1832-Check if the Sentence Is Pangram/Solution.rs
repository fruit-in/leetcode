impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence.bytes().fold(0, |acc, c| match c {
            b'A'..=b'Z' => acc | (1 << (c - b'A')),
            b'a'..=b'z' => acc | (1 << (c - b'a')),
            _ => acc,
        }) == (1 << 26) - 1
    }
}
