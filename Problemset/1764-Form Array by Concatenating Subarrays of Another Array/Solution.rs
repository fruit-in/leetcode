impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;

        for group in groups {
            let mut flag = true;

            loop {
                if i + group.len() > nums.len() {
                    return false;
                }

                let mut j = 0;

                while j < group.len() {
                    if nums[i + j] != group[j] {
                        flag = false;
                        break;
                    }
                    j += 1;
                }

                if flag {
                    i += j;
                    break;
                }

                i += 1;
                flag = true;
            }
        }

        true
    }
}
