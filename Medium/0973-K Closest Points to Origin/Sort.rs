impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_unstable_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points[..(k as usize)].to_vec()
    }
}
