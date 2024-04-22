impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut x = 6;
        let mut y = 6;

        for _ in 1..n {
            x = (x + y) % 1_000_000_007 * 2 % 1_000_000_007;
            y = (x + y) % 1_000_000_007;
        }

        (x + y) % 1_000_000_007
    }
}
