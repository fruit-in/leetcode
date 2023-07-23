impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        directions
            .trim_start_matches('L')
            .trim_end_matches('R')
            .bytes()
            .filter(|&d| d != b'S')
            .count() as i32
    }
}
