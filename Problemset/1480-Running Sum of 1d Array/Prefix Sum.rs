impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sums = nums;

        for i in 1..running_sums.len() {
            running_sums[i] += running_sums[i - 1];
        }

        running_sums
    }
}
