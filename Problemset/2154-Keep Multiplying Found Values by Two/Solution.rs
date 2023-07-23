impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut original = original;
        let mut flag = 0;

        for num in nums {
            if num % original == 0 && (num / original).count_ones() == 1 {
                flag |= num / original;
            }
        }

        while flag & 1 == 1 {
            original *= 2;
            flag >>= 1;
        }

        original
    }
}
