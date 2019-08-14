impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n > 0 {
            let x_n2 = Self::my_pow(x, n / 2);
            if n % 2 == 0 {
                x_n2 * x_n2
            } else {
                x_n2 * x_n2 * x
            }
        } else if n < 0 {
            let x_n2 = Self::my_pow(1.0 / x, -(n / 2));
            if n % 2 == 0 {
                x_n2 * x_n2
            } else {
                x_n2 * x_n2 / x
            }
        } else {
            1.0
        }
    }
}
