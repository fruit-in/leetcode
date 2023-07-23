impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_by_key(|num| {
            num.to_string()
                .bytes()
                .map(|digit| mapping[(digit - b'0') as usize])
                .fold(0, |x, digit| x * 10 + digit)
        });

        nums
    }
}
