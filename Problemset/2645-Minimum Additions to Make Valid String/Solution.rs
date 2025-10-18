impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut want = b'a';
        let mut ret = 0;

        for ch in word.bytes() {
            ret += (ch + 3 - want) as i32 % 3;
            want = (ch - b'a' + 1) % 3 + b'a';
        }

        ret += 2 - (want - b'a' + 2) as i32 % 3;

        ret
    }
}
