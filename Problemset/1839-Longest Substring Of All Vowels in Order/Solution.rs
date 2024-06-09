impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut vowels = "_aeiou_".as_bytes();
        let mut i = 0;
        let mut count = 0;
        let mut ret = 0;

        for c in word.bytes().chain(std::iter::once(b' ')) {
            if c == vowels[i] {
                count += 1;
            } else if c == vowels[i + 1] {
                i += 1;
                count += 1;
            } else {
                if i == 5 {
                    ret = ret.max(count);
                }
                i = (c == b'a') as usize;
                count = i as i32;
            }
        }

        ret
    }
}
