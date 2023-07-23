impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut ret = vec![1, num + 1];

        for n in (num + 1)..(num + 3) {
            for i in (2..=((n as f64).sqrt().floor() as i32)).rev() {
                if n % i == 0 && n / i - i < ret[1] - ret[0] {
                    ret = vec![i, n / i];
                    break;
                }
            }
        }

        ret
    }
}
