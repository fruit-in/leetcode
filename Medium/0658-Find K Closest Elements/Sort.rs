impl Solution {
    pub fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        arr.sort_unstable_by_key(|&y| ((y - x).abs(), y));
        arr.truncate(k as usize);
        arr.sort_unstable();

        arr
    }
}
