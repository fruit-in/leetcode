impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * Self::last_remaining_rev(n / 2)
    }

    pub fn last_remaining_rev(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * Self::last_remaining(n / 2) - 1 + n % 2
    }
}
