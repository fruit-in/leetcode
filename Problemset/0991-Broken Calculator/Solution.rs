impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut y = y;
        let mut ret = 0;

        while x < y {
            ret += 1;

            if y % 2 == 1 {
                y += 1;
            } else {
                y /= 2;
            }
        }

        ret + x - y
    }
}
