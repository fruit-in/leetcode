impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let mut score = a[0];
        let mut ret = 0;

        for j in 1..a.len() {
            ret = ret.max(score + a[j] - j as i32);
            score = score.max(a[j] + j as i32);
        }

        ret
    }
}
