impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for r in 1..triangle.len() {
            triangle[r][0] += triangle[r - 1][0];
            triangle[r][r] += triangle[r - 1][r - 1];
            for i in 1..(triangle[r].len() - 1) {
                triangle[r][i] += triangle[r - 1][i - 1].min(triangle[r - 1][i])
            }
        }

        *triangle.last().unwrap().iter().min().unwrap()
    }
}
