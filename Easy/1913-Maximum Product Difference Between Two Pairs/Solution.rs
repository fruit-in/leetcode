impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut max = (i32::MIN, i32::MIN);
        let mut min = (i32::MAX, i32::MAX);

        for num in nums {
            if num >= max.0 {
                max = (num, max.0);
            } else if num > max.1 {
                max.1 = num;
            }
            if num <= min.0 {
                min = (num, min.0)
            } else if num < min.1 {
                min.1 = num;
            }
        }

        max.0 * max.1 - min.0 * min.1
    }
}
