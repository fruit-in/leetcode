impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.into_bytes();
        let mut score_l = (b'1' - s[0]) as i32;
        let mut score_r = s[1..].iter().filter(|&&ch| ch == b'1').count() as i32;
        let mut ret = score_l + score_r;

        for i in 1..(s.len() - 1) {
            match s[i] {
                b'0' => score_l += 1,
                _ => score_r -= 1,
            }
            ret = ret.max(score_l + score_r);
        }

        ret
    }
}
