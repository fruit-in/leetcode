impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = (0..profit.len())
            .map(|i| (end_time[i], start_time[i], profit[i]))
            .collect::<Vec<_>>();
        let mut dp = vec![(0, 0)];
        let mut ret = 0;
        jobs.sort_unstable();

        for &(et, st, pf) in &jobs {
            let i = match dp.binary_search_by_key(&st, |&(t, _)| t) {
                Ok(i) => i,
                Err(i) => i - 1,
            };

            if et != dp.last().unwrap().0 {
                dp.push((et, dp[i].1 + pf));
            } else if dp.last().unwrap().1 < dp[i].1 + pf {
                dp.last_mut().unwrap().1 = dp[i].1 + pf;
            }

            if ret > dp.last().unwrap().1 {
                dp.last_mut().unwrap().1 = ret;
            } else {
                ret = dp.last().unwrap().1;
            }
        }

        ret
    }
}
