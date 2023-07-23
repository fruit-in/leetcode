impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        while num >= 10 {
            let mut tmp = 0;
            while num != 0 {
                tmp += num % 10;
                num /= 10;
            }
            num = tmp;
        }
        num
    }
}
