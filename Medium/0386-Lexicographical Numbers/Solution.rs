impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![];

        for _ in 0..n {
            ret.push(x);

            if x * 10 <= n {
                x *= 10;
            } else if x % 10 == 9 || x == n {
                x /= 10;
                while x % 10 == 9 {
                    x /= 10;
                }
                x += 1;
            } else {
                x += 1;
            }
        }

        ret
    }
}
