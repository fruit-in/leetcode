impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        if index < n - index - 1 {
            return Self::max_value(n, n - index - 1, max_sum);
        }

        let n = n as i64;
        let index = index as i64;
        let max_sum = max_sum as i64 - n;
        let mut min = 0;
        let mut max = max_sum;

        while min < max {
            let x = (min + max + 1) / 2;
            let sum = if x > index {
                ((2 * x - index) * (index + 1) + (2 * x - n + index) * (n - 1 - index)) / 2
            } else if x > n - 1 - index {
                ((1 + x) * x + (2 * x - n + index) * (n - 1 - index)) / 2
            } else {
                x * x
            };

            if sum <= max_sum {
                min = x;
            } else {
                max = x - 1;
            }
        }

        min as i32 + 1
    }
}
