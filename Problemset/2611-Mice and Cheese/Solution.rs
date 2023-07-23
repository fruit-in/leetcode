impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut indices = (0..reward1.len()).collect::<Vec<_>>();
        let mut ret = 0;

        indices.sort_unstable_by_key(|&i| reward2[i] - reward1[i]);

        for i in 0..reward1.len() {
            if i < k as usize {
                ret += reward1[indices[i]];
            } else {
                ret += reward2[indices[i]];
            }
        }

        ret
    }
}
