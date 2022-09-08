impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();

        if amount[0] + amount[1] < amount[2] {
            amount[2]
        } else {
            (amount.iter().sum::<i32>() + 1) / 2
        }
    }
}
