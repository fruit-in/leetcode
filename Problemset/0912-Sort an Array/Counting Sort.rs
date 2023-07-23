impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let mut count = vec![0; (max - min) as usize + 1];
        let mut new_array = Vec::new();
        for n in nums {
            count[(n - min) as usize] += 1;
        }
        for i in 0..=(max - min) {
            for j in 0..count[i as usize] {
                new_array.push(min + i);
            }
        }
        new_array
    }
}
