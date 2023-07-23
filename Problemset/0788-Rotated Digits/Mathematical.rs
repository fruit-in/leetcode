impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        fn helper(n: i32, allow_same: bool) -> i32 {
            let x = (n as f64).log10() as u32;
            let a = n / 10_i32.pow(x);
            let b = n % 10_i32.pow(x);

            match (a, allow_same) {
                (1, true) => 7_i32.pow(x) + helper(b, true),
                (2, true) => 2 * 7_i32.pow(x) + helper(b, true),
                (3, true)|(4, true) => 3 * 7_i32.pow(x),
                (5, true) => 3 * 7_i32.pow(x) + helper(b, true),
                (6, true) => 4 * 7_i32.pow(x) + helper(b, true),
                (7, true) => 5 * 7_i32.pow(x),
                (8, true) => 5 * 7_i32.pow(x) + helper(b, true),
                (9, true) => 6 * 7_i32.pow(x) + helper(b, true),
                (_, true) => 1,

                (1, false) => 7_i32.pow(x) - 3_i32.pow(x) + helper(b, false),
                (2, false) => 2 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (3, false)|(4, false) => 3 * 7_i32.pow(x) - 2 * 3_i32.pow(x),
                (5, false) => 3 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (6, false) => 4 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (7, false) => 5 * 7_i32.pow(x) - 2 * 3_i32.pow(x),
                (8, false) => 5 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, false),
                (9, false) => 6 * 7_i32.pow(x) - 3 * 3_i32.pow(x) + helper(b, true),
                (_, false) => 0,
            }
        }

        helper(n, false)
    }
}
