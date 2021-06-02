impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut work = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .collect::<Vec<_>>();
        work.sort_unstable();
        let mut ret = 0;

        for i in 1..work.len() {
            work[i].1 = work[i].1.max(work[i - 1].1);
        }

        for w in worker {
            ret += match work.binary_search_by_key(&w, |&(d, _)| d) {
                Ok(i) => work[i].1,
                Err(0) => 0,
                Err(i) => work[i - 1].1,
            };
        }

        ret
    }
}
