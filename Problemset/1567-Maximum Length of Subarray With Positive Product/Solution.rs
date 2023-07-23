impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for v in nums.split(|&num| num == 0) {
            let mut tmp = v.len();

            if v.iter().filter(|&&x| x < 0).count() % 2 == 1 {
                let l_ne = v.iter().position(|&x| x < 0);
                let r_ne = v.iter().rev().position(|&x| x < 0);

                tmp -= l_ne.min(r_ne).unwrap() + 1;
            }

            ret = ret.max(tmp);
        }

        ret as i32
    }
}
