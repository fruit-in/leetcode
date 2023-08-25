impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = s.iter().filter(|&&c| c == b'0').count() as i32;
        let mut ret = count;

        for i in 0..s.len() {
            count += (s[i] == b'1') as i32 - (s[i] == b'0') as i32;

            ret = ret.min(count);
        }

        ret
    }
}
