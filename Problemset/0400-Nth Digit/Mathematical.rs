impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n;
        let mut a = 0;
        let mut b = 0;
        let mut c = 9;

        for i in 0..8 {
            a += 9 * (i + 1) * 10_i32.pow(i as u32);
            if a >= n {
                c = i + 1;
                break;
            }
            b = a;
        }

        n -= b + 1;
        match n % c {
            0 => 1 + n / (c * 10_i32.pow(c as u32 - 1)),
            d => n % (c * 10_i32.pow(c as u32 - d as u32)) / (c * 10_i32.pow(c as u32 - d as u32 - 1)),
        }
    }
}
