impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let expensive = cost1.max(cost2);
        let cheap = cost1.min(cost2);

        (0..=total)
            .step_by(expensive as usize)
            .map(|x| ((total - x) / cheap) as i64 + 1)
            .sum()
    }
}
