impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut height = 0;
        let mut ret = 0;

        for rung in rungs {
            if rung - height > dist {
                ret += (rung - height - 1) / dist;
            }
            height = rung;
        }

        ret
    }
}
