impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let x = if x_center >= x1 && x_center <= x2 {
            0
        } else {
            (x_center - x1).abs().min((x_center - x2).abs())
        };
        let y = if y_center >= y1 && y_center <= y2 {
            0
        } else {
            (y_center - y1).abs().min((y_center - y2).abs())
        };

        x * x + y * y <= radius * radius
    }
}
