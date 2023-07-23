impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut n_sum = mean * (rolls.len() as i32 + n) - rolls.into_iter().sum::<i32>();
        let mut ret = vec![];

        if n_sum < n || n_sum > 6 * n {
            return ret;
        }

        for i in (0..n).rev() {
            let x = (n_sum - 6 * i).max(1);
            n_sum -= x;
            ret.push(x);
        }

        ret
    }
}
