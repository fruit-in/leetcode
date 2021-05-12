impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .enumerate()
            .filter(|(_, p)| p[0] == x || p[1] == y)
            .map(|(i, p)| (i, (p[0] - x).abs() + (p[1] - y).abs()))
            .min_by_key(|&(_, d)| d)
            .map_or(-1, |(i, _)| i as i32)
    }
}
