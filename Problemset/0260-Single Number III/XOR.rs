impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let acc = nums.iter().fold(0, |acc, n| acc ^ n);
        let mask = acc & (-acc);
        let mut ret = vec![0, 0];

        for n in nums {
            match n & mask {
                0 => ret[0] ^= n,
                _ => ret[1] ^= n,
            }
        }

        ret
    }
}
