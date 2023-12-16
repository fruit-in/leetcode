impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let digits = n
            .to_string()
            .bytes()
            .map(|x| (x - b'0') as i32)
            .collect::<Vec<_>>();
        let mut tenpow_r = 10_i32.pow(digits.len() as u32 - 1);
        let mut part_l = 0;
        let mut part_r = n;
        let mut ret = 0;

        for &digit in &digits {
            part_r = n % tenpow_r;
            ret += part_l * tenpow_r
                + match digit {
                    0 => 0,
                    1 => part_r + 1,
                    _ => tenpow_r,
                };
            tenpow_r /= 10;
            part_l = part_l * 10 + digit;
        }

        ret
    }
}
