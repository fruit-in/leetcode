impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let word = word.as_bytes();
        let mut x = 0;
        let mut ret = 0;

        for i in 0..word.len() {
            if [b'a', b'e', b'i', b'o', b'u'].contains(&word[i]) {
                x += i as i64 + 1;
            }

            ret += x;
        }

        ret
    }
}
