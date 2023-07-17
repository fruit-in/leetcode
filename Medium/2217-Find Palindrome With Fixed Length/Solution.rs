impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let mut ret = Vec::with_capacity(queries.len());

        for &query in &queries {
            let half_length = (int_length as u32 + 1) / 2;

            if query >= 9 * 10_i32.pow(half_length - 1) + 1 {
                ret.push(-1);
                continue;
            }

            let mut x = query as i64 + 10_i64.pow(half_length - 1) - 1;
            let mut y = x;

            if int_length % 2 == 1 {
                y /= 10;
            }

            for _ in 0..int_length / 2 {
                x = x * 10 + y % 10;
                y /= 10;
            }

            ret.push(x);
        }

        ret
    }
}
