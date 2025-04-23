impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = t.len() - 1;
        let mut matches_right = vec![(t.len(), s.len())];
        let mut ret = t.len();

        for j in (0..s.len()).rev() {
            if s[j] == t[i] {
                matches_right.push((i, j));
                ret = i;

                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }

        i = 0;

        for j in 0..s.len() {
            if s[j] == t[i] {
                while matches_right.last().unwrap_or(&(0, s.len())).1 <= j {
                    matches_right.pop();
                }
                ret = ret.min(
                    matches_right
                        .last()
                        .unwrap_or(&(t.len(), 0))
                        .0
                        .saturating_sub(i + 1),
                );

                if i == t.len() - 1 {
                    break;
                } else {
                    i += 1;
                }
            }
        }

        ret as i32
    }
}
