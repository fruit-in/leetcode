impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let sum_a = a.iter().sum::<i32>();
        let sum_b = b.iter().sum::<i32>();

        for exchange_a in a {
            let exchange_b = (sum_b - sum_a) / 2 + exchange_a;
            if b.contains(&exchange_b) {
                return vec![exchange_a, exchange_b];
            }
        }

        Vec::new()
    }
}
