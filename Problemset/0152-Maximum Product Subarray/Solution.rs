impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ret = *nums.iter().max().unwrap();

        for slice in nums.split(|&num| num == 0) {
            if slice.iter().filter(|&&num| num < 0).count() % 2 == 1 {
                let i = slice.iter().position(|&num| num < 0).unwrap();
                let j = slice.iter().rposition(|&num| num < 0).unwrap();

                if i + 1 < slice.len() {
                    ret = ret.max(slice.iter().skip(i + 1).product());
                }
                if j > 0 {
                    ret = ret.max(slice.iter().take(j).product());
                }
            } else if !slice.is_empty() {
                ret = ret.max(slice.iter().product());
            }
        }

        ret
    }
}
