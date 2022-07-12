impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        let mut ret = 0;
        cost.sort_unstable_by(|a, b| b.cmp(a));

        for candies3 in cost.chunks(3) {
            ret += candies3[0];
            ret += candies3.get(1).unwrap_or(&0);
        }

        ret
    }
}
