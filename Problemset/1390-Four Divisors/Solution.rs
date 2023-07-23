impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for &x in &nums {
            let mut tmp = 0;

            for y in 2..=(x as f64).sqrt() as i32 {
                if x % y == 0 {
                    if tmp > 0 || x == y * y {
                        tmp = 0;
                        break;
                    }
                    tmp += y + x / y;
                }
            }

            if tmp > 0 {
                ret += 1 + x + tmp;
            }
        }

        ret
    }
}
