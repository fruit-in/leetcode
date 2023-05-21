impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut x = 1;

        while 4 * x * x * x + 6 * x * x + 2 * x < needed_apples {
            x += 1;
        }

        8 * x
    }
}
