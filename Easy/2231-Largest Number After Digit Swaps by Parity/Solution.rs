impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut num = num;
        let mut odd_digits = vec![];
        let mut even_digits = vec![];
        let mut is_odds = vec![];
        let mut ret = 0;

        while num > 0 {
            if num % 2 == 1 {
                odd_digits.push(num % 10);
                is_odds.push(true);
            } else {
                even_digits.push(num % 10);
                is_odds.push(false);
            }
            num /= 10;
        }

        odd_digits.sort_unstable();
        even_digits.sort_unstable();

        while let Some(is_odd) = is_odds.pop() {
            if is_odd {
                ret = ret * 10 + odd_digits.pop().unwrap();
            } else {
                ret = ret * 10 + even_digits.pop().unwrap();
            }
        }

        ret
    }
}
