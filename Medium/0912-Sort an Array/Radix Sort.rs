impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            nums[i] += 50000;
        }
        let mut bucket = vec![Vec::new(); 10];
        for i in 0..5 {
            for n in nums {
                let m = n % 10_i32.pow(i + 1) / 10_i32.pow(i);
                bucket[m as usize].push(n);
            }
            nums = Vec::new();
            for j in 0..10 {
                while bucket[j].len() > 0 {
                    nums.push(bucket[j].remove(0));
                }
            }
        }
        for i in 0..nums.len() {
            nums[i] -= 50000;
        }
        nums
    }
}
