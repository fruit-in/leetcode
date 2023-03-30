impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut count = vec![0; value as usize];

        for &num in &nums {
            count[num.rem_euclid(value) as usize] += 1;
        }

        for num in 0..=nums.len() as i32 {
            if count[num.rem_euclid(value) as usize] == 0 {
                return num;
            }

            count[num.rem_euclid(value) as usize] -= 1;
        }

        unreachable!();
    }
}
