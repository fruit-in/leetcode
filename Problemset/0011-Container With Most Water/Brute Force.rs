impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut most = 0;
        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                most = most.max((j - i) as i32 * height[i].min(height[j]));
            }
        }
        most
    }
}
