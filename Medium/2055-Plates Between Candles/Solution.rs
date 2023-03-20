impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let candles = s
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'|')
            .map(|(i, _)| i as i32)
            .collect::<Vec<_>>();
        let mut answer = vec![0; queries.len()];

        for i in 0..answer.len() {
            let j = match candles.binary_search(&queries[i][0]) {
                Ok(x) | Err(x) => x,
            };
            let k = match candles.binary_search(&(queries[i][1] + 1)) {
                Ok(x) | Err(x) => x.saturating_sub(1),
            };

            if j < k && k < candles.len() {
                answer[i] = candles[k] - candles[j] - (k - j) as i32;
            }
        }

        answer
    }
}
