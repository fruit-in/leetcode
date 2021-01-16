impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        points.sort_unstable_by_key(|p| p[0]);

        for i in 1..points.len() {
            ret = ret.max(points[i][0] - points[i - 1][0]);
        }

        ret
    }
}
