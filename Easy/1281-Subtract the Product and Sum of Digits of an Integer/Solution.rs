impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::with_capacity(6);

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.iter().product::<i32>() - digits.iter().sum::<i32>()
    }
}
