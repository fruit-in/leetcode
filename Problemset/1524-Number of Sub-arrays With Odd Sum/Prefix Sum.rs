impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut odd = 0;
        let mut even = 1;
        let mut ret = 0;

        for x in arr {
            sum += x;
            if sum % 2 == 1 {
                odd += 1;
                ret = (ret + even) % 1_000_000_007;
            } else {
                even += 1;
                ret = (ret + odd) % 1_000_000_007;
            }
        }

        ret
    }
}
