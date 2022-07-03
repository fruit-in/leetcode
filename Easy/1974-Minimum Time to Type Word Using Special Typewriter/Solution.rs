impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut curr = b'a' as i32;
        let mut ret = 0;

        for c in word.bytes() {
            let (s, l) = (curr.min(c as i32), curr.max(c as i32));
            ret += 1 + (l - s).min(s + 26 - l);
            curr = c as i32;
        }

        ret
    }
}
