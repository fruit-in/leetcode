impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut ret = 0;

        for circle in &circles {
            min_x = min_x.min(circle[0] - circle[2]);
            min_y = min_y.min(circle[1] - circle[2]);
            max_x = max_x.max(circle[0] + circle[2]);
            max_y = max_y.max(circle[1] + circle[2]);
        }

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for circle in &circles {
                    if (circle[0] - x).pow(2) + (circle[1] - y).pow(2) <= circle[2].pow(2) {
                        ret += 1;
                        break;
                    }
                }
            }
        }

        ret
    }
}
