impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;

        loop {
            p1 = nums[p1 as usize];
            p2 = nums[nums[p2 as usize] as usize];
            if p1 == p2 {
                break;
            }
        }

        p1 = 0;
        while p1 != p2 {
            p1 = nums[p1 as usize];
            p2 = nums[p2 as usize];
        }

        p1
    }
}
