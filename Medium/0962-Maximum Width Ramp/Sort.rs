impl Solution {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut v = a
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();
        let mut min_i = a.len();
        let mut ret = 0;

        v.sort_unstable();

        for (_, i) in v {
            ret = ret.max(i.saturating_sub(min_i));
            min_i = min_i.min(i);
        }

        ret as i32
    }
}
