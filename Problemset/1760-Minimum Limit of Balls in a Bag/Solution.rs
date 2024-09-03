impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut low = 1;
        let mut high = *nums.iter().max().unwrap();

        while low < high {
            let mid = (low + high) / 2;
            let mut operations = 0;

            for x in &nums {
                operations += (x - 1) / mid;
            }

            if operations > max_operations {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        high
    }
}
