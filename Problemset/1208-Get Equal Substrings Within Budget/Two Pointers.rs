impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = -1;
        let mut cost = 0;
        let mut ret = 0;

        for j in 0..s.len() {
            cost += (s[j] as i32 - t[j] as i32).abs();
            while cost > max_cost {
                i += 1;
                cost -= (s[i as usize] as i32 - t[i as usize] as i32).abs();
            }
            ret = ret.max(j as i32 - i);
        }

        ret
    }
}
