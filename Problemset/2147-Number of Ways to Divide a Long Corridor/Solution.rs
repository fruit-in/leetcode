impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut count_s = 0;
        let mut count_p = vec![];

        for c in corridor.chars() {
            if c == 'S' {
                count_s += 1;
                if count_s % 2 == 0 {
                    count_p.push(0_i64);
                }
            } else if count_s > 0 && count_s % 2 == 0 {
                *count_p.last_mut().unwrap() += 1;
            }
        }

        if count_s == 0 || count_s % 2 == 1 {
            return 0;
        }

        count_p.pop();

        count_p
            .iter()
            .fold(1, |acc, x| acc * (x + 1) % 1_000_000_007) as i32
    }
}
