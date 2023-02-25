impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let volume = length as i64 * width as i64 * height as i64;
        let is_bulky = length.max(width).max(height) >= 10000 || volume >= 1_000_000_000;
        let is_heavy = mass >= 100;

        match (is_bulky, is_heavy) {
            (true, true) => "Both",
            (false, false) => "Neither",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
        }
        .to_string()
    }
}
