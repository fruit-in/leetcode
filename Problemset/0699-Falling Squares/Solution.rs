impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heights = vec![(
            positions[0][0],
            positions[0][0] + positions[0][1],
            positions[0][1],
        )];
        let mut ans = vec![0; positions.len()];
        ans[0] = positions[0][1];

        for i in 1..positions.len() {
            let left = positions[i][0];
            let length = positions[i][1];
            let right = left + length;
            let mut height = length;

            for j in 0..heights.len() {
                if heights[j].1 > left && heights[j].0 < right {
                    height = height.max(heights[j].2 + length);
                }
            }

            heights.push((left, right, height));
            ans[i] = ans[i - 1].max(height);
        }

        ans
    }
}
