impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut loss = 0;
        let mut max_cost = 0;

        for t in &transactions {
            loss += (t[0] - t[1]).max(0) as i64;
            max_cost = max_cost.max(t[0] - (t[0] - t[1]).max(0));
        }

        max_cost as i64 + loss
    }
}
