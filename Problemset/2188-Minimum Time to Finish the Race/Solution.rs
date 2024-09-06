impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let mut laps_min = vec![i32::MAX; 18.min(num_laps as usize + 1)];
        let mut dp = vec![i32::MAX; num_laps as usize + 1];
        dp[0] = -change_time;

        for i in 0..tires.len() {
            let (f, r) = (tires[i][0], tires[i][1]);
            let mut total_time = change_time;
            let mut lap_time = f;

            for x in 1..laps_min.len() {
                total_time += lap_time;
                laps_min[x] = laps_min[x].min(total_time);
                lap_time = lap_time.saturating_mul(r);
                if lap_time >= change_time + f {
                    break;
                }
            }
        }

        for i in 0..dp.len() {
            for x in 1..laps_min.len() {
                if i + x < dp.len() {
                    dp[i + x] = dp[i + x].min(dp[i].saturating_add(laps_min[x]));
                }
            }
        }

        *dp.last().unwrap()
    }
}
