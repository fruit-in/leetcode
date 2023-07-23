impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut a = p;
        let mut b = q;

        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }

        match (p / a % 2, q / a % 2) {
            (0, _) => 2,
            (1, 0) => 0,
            (_, _) => 1,
        }
    }
}
