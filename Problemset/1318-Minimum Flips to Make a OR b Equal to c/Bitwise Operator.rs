impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut ret = 0;

        while a > 0 || b > 0 || c > 0 {
            let d = a & 1;
            let e = b & 1;
            let f = c & 1;
            match (d << 2) + (e << 1) + f {
                0b001 | 0b010 | 0b100 => ret += 1,
                0b110 => ret += 2,
                _ => (),
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }

        ret
    }
}
