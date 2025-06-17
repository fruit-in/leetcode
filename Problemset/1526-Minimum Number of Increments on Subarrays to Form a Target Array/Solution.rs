impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut ret = target[0];

        for i in 1..target.len() {
            ret += 0.max(target[i] - target[i - 1]);
        }

        ret
    }
}
