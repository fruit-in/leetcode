impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x0 = points[0][0];
        let y0 = points[0][1];
        let x1 = points[1][0];
        let y1 = points[1][1];
        let x2 = points[2][0];
        let y2 = points[2][1];

        (x1 - x0) * (y2 - y0) != (x2 - x0) * (y1 - y0)
    }
}
