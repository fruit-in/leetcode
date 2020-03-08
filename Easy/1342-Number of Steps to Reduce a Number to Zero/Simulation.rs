impl Solution {
    pub fn number_of_steps (num: i32) -> i32 {
        let mut num = num;
        let mut ret = 0;

        while num != 0 {
            match num % 2 {
                0 => num /= 2,
                _ => num -= 1,
            }
            ret += 1;
        }

        ret
    }
}
