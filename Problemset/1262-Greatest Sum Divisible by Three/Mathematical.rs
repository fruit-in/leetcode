impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut min = vec![(19996, 10000), (20000, 10001)];
        let mut ret = 0;

        for num in nums {
            ret += num;
            match num as usize % 3 {
                0 => (),
                i => {
                    if num < min[i - 1].0 {
                        min[i - 1] = (num, min[i - 1].0);
                    } else if num < min[i - 1].1 {
                        min[i - 1].1 = num;
                    }
                },
            }
        }

        match ret % 3 {
            1 => ret - min[0].0.min(min[1].0 + min[1].1),
            2 => ret - min[1].0.min(min[0].0 + min[0].1),
            _ => ret,
        }
    }
}
