impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }

        let mut l = 0;
        let mut r = num;
        let mut m = (l + r) / 2;

        while l <= r && m != num / m {
            if m < num / m {
                l = m + 1;
            } else if m > num / m {
                r = m - 1;
            }
            m = (l + r) / 2;
        }

        m * m == num
    }
}
