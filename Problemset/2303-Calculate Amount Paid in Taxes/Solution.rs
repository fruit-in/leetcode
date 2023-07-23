impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut ret = income.min(brackets[0][0]) * brackets[0][1];

        for i in 1..brackets.len() {
            ret += (income.min(brackets[i][0]) - brackets[i - 1][0]).max(0) * brackets[i][1];
        }

        ret as f64 / 100.
    }
}
