impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        match (n, presses) {
            (_, 0) => 1,
            (1, _) => 2,
            (2, 1) => 3,
            (2, _) => 4,
            (_, 1) => 4,
            (_, 2) => 7,
            _ => 8,
        }
    }
}
