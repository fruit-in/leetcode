impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let mut start = 0;
        let mut cnt = 0;
        while cnt < nums.len() {
            let mut cur = start;
            let mut prev = nums[cur];
            loop {
                let next = (cur + k) % nums.len();
                let temp = nums[next];
                nums[next] = prev;
                cur = next;
                prev = temp;
                cnt += 1;
                
                if start == cur {
                    break;
                }
            }
            start += 1;
        }
    }
}
