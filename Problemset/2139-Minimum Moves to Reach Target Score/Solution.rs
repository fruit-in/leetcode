impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut target = target;
        let mut max_doubles = max_doubles;
        let mut ret = 0;

        while target > 1 {
            if max_doubles == 0 {
                return ret + target - 1;
            } else if target % 2 == 1 {
                target -= 1;
            } else {
                target /= 2;
                max_doubles -= 1;
            }

            ret += 1;
        }

        ret
    }
}
