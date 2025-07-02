use std::collections::VecDeque;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut mask1 = VecDeque::from(vec![0; n]);
        let mut mask2 = VecDeque::from(vec![0; n]);
        let mut ret = 0;

        for i in 0..n {
            for j in 0..n {
                mask1[i] |= img1[i][j] << j;
                mask2[i] |= img2[i][j] << j;
            }
        }

        let mut tmp = mask1.clone();

        for _ in 0..n {
            for i in 0..n {
                let mut left = 0;
                let mut right = 0;

                for j in 0..n {
                    left += ((tmp[j] << i) & mask2[j]).count_ones();
                    right += ((tmp[j] >> i) & mask2[j]).count_ones();
                }

                ret = ret.max(left).max(right);
            }

            tmp[0] = 0;
            tmp.rotate_left(1);
        }

        tmp = mask1.clone();

        for _ in 0..n {
            for i in 0..n {
                let mut left = 0;
                let mut right = 0;

                for j in 0..n {
                    left += ((tmp[j] << i) & mask2[j]).count_ones();
                    right += ((tmp[j] >> i) & mask2[j]).count_ones();
                }

                ret = ret.max(left).max(right);
            }

            tmp.rotate_right(1);
            tmp[0] = 0;
        }

        ret as i32
    }
}
