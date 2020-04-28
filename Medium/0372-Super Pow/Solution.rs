impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let a = a % 1337;
        let mut b = b;
        let mut ret = 1;

        let tmp = match b.pop() {
            Some(n) => {
                for _ in 0..n {
                    ret *= a;
                    ret %= 1337;
                }
                Self::super_pow(a, b)
            },
            None => 1,
        };

        for _ in 0..10 {
            ret *= tmp;
            ret %= 1337;
        }

        ret
    }
}
