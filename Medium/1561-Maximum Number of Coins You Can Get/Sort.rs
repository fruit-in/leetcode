impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let n = piles.len() / 3;
        let mut piles = piles;
        piles.sort_unstable();

        piles.iter().skip(n).step_by(2).sum()
    }
}
