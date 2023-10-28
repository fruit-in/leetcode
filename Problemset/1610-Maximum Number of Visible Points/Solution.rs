impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let angle = (angle as f64).to_radians();
        let origin = points.iter().filter(|&p| *p == location).count();
        let mut points = points
            .iter()
            .filter(|&p| *p != location)
            .map(|p| ((p[1] - location[1]) as f64).atan2((p[0] - location[0]) as f64))
            .collect::<Vec<_>>();
        let mut i = 0;
        let mut max_count = 0;

        points.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        for j in 0..points.len() {
            points.push(points[j] + 2.0 * std::f64::consts::PI);
        }

        for j in 0..points.len() / 2 {
            while i < points.len() && points[i] - points[j] <= angle {
                i += 1;
            }

            max_count = max_count.max(i - j);
        }

        (origin + max_count) as i32
    }
}
