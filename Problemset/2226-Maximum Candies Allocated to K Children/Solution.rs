impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut l = 1;
        let mut r = 10_000_001;

        while l < r {
            let m = (l + r) / 2;
            let count = candies.iter().map(|&x| (x / m) as i64).sum::<i64>();

            if count < k {
                r = m;
            } else {
                l = m + 1;
            }
        }

        r - 1
    }
}
