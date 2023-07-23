impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[1]);
        let mut x = points[0][1];
        let mut ret = 1;

        for i in 1..points.len() {
            if points[i][0] > x {
                x = points[i][1];
                ret += 1;
            }
        }

        ret
    }
}
