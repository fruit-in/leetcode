impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let n = n as i64;
        let mut lo = 1;
        let mut hi = n;

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut boxes = 0;
            let mut x = (mid as f64 * 2.).sqrt() as i64;
            let mut y = mid - x * (x + 1) / 2;

            if y < 0 {
                y += x;
                x -= 1;
            }

            while x > 0 {
                boxes += x * (x + 1) / 2 + y;
                x -= 1;
                y -= 1;
                y = y.max(0);
            }

            if boxes < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        hi as i32
    }
}
