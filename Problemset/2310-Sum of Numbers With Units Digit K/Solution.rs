impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        let mut sum = k;

        for i in 1..=10 {
            if sum <= num && sum % 10 == num % 10 {
                return i;
            }
            sum += k;
        }

        -1
    }
}
