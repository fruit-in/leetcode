impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = b.len() - 1;
        let mut indices = (0..b.len()).collect::<Vec<_>>();
        let mut ret = vec![0; b.len()];

        a.sort_unstable();
        indices.sort_unstable_by_key(|&k| -b[k]);

        for k in indices {
            if b[k] < a[j] {
                ret[k] = a[j];
                j -= 1;
            } else {
                ret[k] = a[i];
                i += 1;
            }
        }

        ret
    }
}
