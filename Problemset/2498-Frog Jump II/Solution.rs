impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        let mut lo = 1;
        let mut hi = *stones.last().unwrap();

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut i = 0;
            let mut j = 1;
            let mut flag = true;
            let mut used = vec![false; stones.len()];

            while i < stones.len() - 1 {
                if stones[j] - stones[j - 1] > mid {
                    flag = false;
                    break;
                } else if j == stones.len() - 1 || stones[j + 1] - stones[i] > mid {
                    i = j;
                    used[i] = true;
                }

                j += 1;
            }

            if flag {
                used[i] = false;
                i = 0;
                j = 1;

                while i < stones.len() - 1 {
                    if stones[j] - stones[i] > mid {
                        flag = false;
                        break;
                    } else if !used[j] && stones[j] - stones[i] <= mid {
                        i = j;
                    }

                    j += 1;
                }
            }

            if flag {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        hi
    }
}
