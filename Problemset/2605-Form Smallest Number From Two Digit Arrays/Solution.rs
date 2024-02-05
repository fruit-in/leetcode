impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min0 = 10;
        let min1 = *nums1.iter().min().unwrap();
        let min2 = *nums2.iter().min().unwrap();

        for x in &nums1 {
            for y in &nums2 {
                if x == y {
                    min0 = min0.min(*x);
                }
            }
        }

        if min0 < 10 {
            min0
        } else if min1 < min2 {
            min1 * 10 + min2
        } else {
            min2 * 10 + min1
        }
    }
}
