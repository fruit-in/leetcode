impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut lo = 1;
        let mut hi = *piles.iter().max().unwrap();

        while lo < hi {
            let k = (lo + hi) / 2;
            let hours = piles.iter().map(|&x| ((x - 1) / k + 1) as i64).sum::<i64>();

            if hours > h as i64 {
                lo = k + 1;
            } else {
                hi = k;
            }
        }

        lo
    }
}
