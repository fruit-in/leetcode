impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = vec![];
        let mut answers = vec![1_i64; queries.len()];

        for i in 0..30 {
            if n & (1 << i) != 0 {
                powers.push(1 << i);
            }
        }

        for i in 0..queries.len() {
            for j in queries[i][0] as usize..=queries[i][1] as usize {
                answers[i] = (answers[i] * powers[j]) % 1_000_000_007;
            }
        }

        answers.into_iter().map(|x| x as i32).collect()
    }
}
