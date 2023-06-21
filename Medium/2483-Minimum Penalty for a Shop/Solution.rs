impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut count_n = 0;
        let mut count_y = customers.chars().filter(|&c| c == 'Y').count();
        let mut min_penalty = count_n + count_y;
        let mut ret = 0;

        for (i, c) in customers.chars().enumerate() {
            if c == 'N' {
                count_n += 1;
            } else if c == 'Y' {
                count_y -= 1;
            }

            if min_penalty > count_n + count_y {
                min_penalty = count_n + count_y;
                ret = i + 1;
            }
        }

        ret as i32
    }
}
