impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];

        for num in nums {
            count[num as usize] += 1;
        }

        count
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 1)
            .map(|(n, _)| n as i32)
            .sum()
    }
}
