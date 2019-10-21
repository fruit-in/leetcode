impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ret = 0;

        for ch in s.bytes() {
            ret ^= ch;
        }

        for ch in t.bytes() {
            ret ^= ch;
        }

        ret as char
    }
}
