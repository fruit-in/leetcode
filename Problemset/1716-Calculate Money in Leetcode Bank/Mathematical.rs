impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let x = n / 7;
        let y = n % 7;

        28 * x + x * (x - 1) * 7 / 2 + (x + 1) * y + y * (y - 1) / 2
    }
}
