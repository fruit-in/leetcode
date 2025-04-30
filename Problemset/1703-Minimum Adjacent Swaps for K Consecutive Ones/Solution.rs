impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut moves = k / 2 * (k % 2 - 1);
        let mut l = 0;
        let mut m = 0;
        let mut r = 0;
        let mut ret = i32::MAX;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                count += 1;
                if count <= (k - 1) / 2 {
                    moves -= count * 2;
                }
                if count == 1 {
                    l = i;
                }
                if count == (k + 1) / 2 {
                    m = i;
                }
                if count == k {
                    r = i;
                    break;
                }
            }
        }

        for i in l..m {
            if nums[i] == 1 {
                moves += (m - i) as i32;
            }
        }
        for i in m + 1..=r {
            if nums[i] == 1 {
                moves += (i - m) as i32;
            }
        }
        ret = ret.min(moves);

        loop {
            moves += l as i32;
            moves -= m as i32 * (k % 2);
            l += 1;
            m += 1;
            r += 1;
            while r < nums.len() && nums[r] == 0 {
                r += 1;
            }
            if r == nums.len() {
                break;
            }
            while nums[m] == 0 {
                m += 1;
            }
            while nums[l] == 0 {
                l += 1;
            }
            moves += r as i32;
            moves -= m as i32 * (2 - k % 2);
            ret = ret.min(moves);
        }

        ret
    }
}
