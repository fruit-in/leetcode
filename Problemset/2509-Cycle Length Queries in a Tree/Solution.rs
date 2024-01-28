impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = vec![1; queries.len()];

        for i in 0..queries.len() {
            let mut a = queries[i][0];
            let mut b = queries[i][1];

            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            answer[i] += (a.leading_zeros() - b.leading_zeros()) as i32;
            b >>= a.leading_zeros() - b.leading_zeros();
            answer[i] += 2 * (32 - a.leading_zeros() as i32);

            for j in (0..(32 - a.leading_zeros())).rev() {
                if a & (1 << j) == b & (1 << j) {
                    answer[i] -= 2;
                } else {
                    break;
                }
            }
        }

        answer
    }
}
