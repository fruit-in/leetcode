impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut dir = 1;
        let mut ret = 1;

        for _ in 0..time {
            ret += dir;

            if ret == 1 || ret == n {
                dir = -dir;
            }
        }

        ret
    }
}
