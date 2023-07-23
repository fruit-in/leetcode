impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        let mut left = 0;
        let mut right = x;
        let mut mid = (left + right) / 2;
        loop {
            if mid <= x / mid && (mid + 1) > x / (mid + 1) {
                return mid;
            } else if mid > x / mid {
                right = mid;
            } else if mid < x / mid {
                left = mid;
            }
            mid = (left + right) / 2;
        }
    }
}
