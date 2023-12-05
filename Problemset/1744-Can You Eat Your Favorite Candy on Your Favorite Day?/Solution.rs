impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_sum = candies_count
            .into_iter()
            .map(|x| x as i64)
            .collect::<Vec<_>>();
        let mut answer = vec![true; queries.len()];

        for i in 1..prefix_sum.len() {
            prefix_sum[i] += prefix_sum[i - 1];
        }

        for i in 0..answer.len() {
            let favorite_type = queries[i][0] as usize;
            let favorite_day = queries[i][1] as i64;
            let daily_cap = queries[i][2] as i64;

            answer[i] = prefix_sum[favorite_type] > favorite_day
                && (favorite_type == 0
                    || (favorite_day + 1) * daily_cap > prefix_sum[favorite_type - 1]);
        }

        answer
    }
}
