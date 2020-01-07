impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 0..(points.len() - 1) {
            let (x0, y0) = (points[i][0], points[i][1]);
            let (x1, y1) = (points[i + 1][0], points[i + 1][1]);
            ret += (x0 - x1).abs().max((y0 - y1).abs());
        }

        ret
    }
}
