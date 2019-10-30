impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let x0 = &coordinates[0][0];
        let y0 = &coordinates[0][1];
        let x1 = &coordinates[1][0];
        let y1 = &coordinates[1][1];

        for point in &coordinates[2..] {
            let x = point[0];
            let y = point[1];

            if (y - y0) * (x1 - x0) != (x - x0) * (y1 - y0) {
                return false;
            }
        }

        true
    }
}
