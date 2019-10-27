impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x0 = points[0][0] as f64;
        let y0 = points[0][1] as f64;
        let x1 = points[1][0] as f64;
        let y1 = points[1][1] as f64;
        let x2 = points[2][0] as f64;
        let y2 = points[2][1] as f64;

        let a = ((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0)).sqrt();
        let b = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
        let c = ((x2 - x0) * (x2 - x0) + (y2 - y0) * (y2 - y0)).sqrt();

        a + b > c && b + c > a && a + c > b
    }
}
