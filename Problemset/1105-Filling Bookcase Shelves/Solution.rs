impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![i32::MAX; books.len() + 1];
        dp[0] = 0;

        for i in 0..dp.len() {
            let mut w = 0;
            let mut h = 0;

            for j in i + 1..dp.len() {
                w += books[j - 1][0];
                h = h.max(books[j - 1][1]);

                if w > shelf_width {
                    break;
                }

                dp[j] = dp[j].min(dp[i] + h);
            }
        }

        *dp.last().unwrap()
    }
}
