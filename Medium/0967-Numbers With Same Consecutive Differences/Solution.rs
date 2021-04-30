impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut nums = (1..10).collect();

        for _ in 1..n {
            let mut nums_ = vec![];

            for x in nums {
                let y = x % 10;
                if y + k < 10 {
                    nums_.push(x * 10 + y + k);
                }
                if y - k >= 0 && k != 0 {
                    nums_.push(x * 10 + y - k);
                }
            }

            nums = nums_;
        }

        nums
    }
}
