impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut score = card_points.iter().take(k).sum::<i32>();
        let mut ret = score;

        for i in 1..=k {
            score += card_points[card_points.len() - i] - card_points[k - i];
            ret = ret.max(score);
        }

        ret
    }
}
