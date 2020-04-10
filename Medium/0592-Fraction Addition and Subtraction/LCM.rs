impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut numerator = 0;
        let mut denominator = 1;
        let mut sum = 0;
        let mut is_numerator = true;
        let mut sign = 1;

        for ch in expression.bytes() {
            match ch {
                b'+' => {
                    sum += sign * numerator * (2520 / denominator);
                    numerator = 0;
                    is_numerator = true;
                    sign = 1;
                },
                b'-' => {
                    sum += sign * numerator * (2520 / denominator);
                    numerator = 0;
                    is_numerator = true;
                    sign = -1;
                },
                b'/' => {
                    is_numerator = false;
                    denominator = 0;
                },
                n if is_numerator => {
                    numerator *= 10;
                    numerator += (n - b'0') as i32;
                },
                n => {
                    denominator *= 10;
                    denominator += (n - b'0') as i32;
                }
            }
        }

        sum += sign * numerator * (2520 / denominator);
        denominator = 2520;

        for i in &[2, 2, 2, 3, 3, 5, 7] {
            if sum % i == 0 {
                sum /= i;
                denominator /= i;
            }
        }

        sum.to_string() + "/" + &denominator.to_string()
    }
}
