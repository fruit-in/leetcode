impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits = n
            .to_string()
            .bytes()
            .map(|ch| (ch - b'0') as i32)
            .collect::<Vec<_>>();
        let mut ret = n;

        for i in 0..digits.len() - 1 {
            let mut x = 9;

            for j in (10 - i as i32)..=9 {
                x *= j;
            }

            ret -= x;
        }

        for i in 0..digits.len() {
            let mut count = (i == 0) as i32;

            for j in 0..i {
                if digits[j] < digits[i] {
                    count += 1;
                }
            }

            let mut x = digits[i] - count;

            for j in (11 - digits.len() as i32)..(10 - i as i32) {
                x *= j;
            }

            ret -= x;

            if digits[..i].contains(&digits[i]) {
                break;
            }

            if i == digits.len() - 1 {
                ret -= 1;
            }
        }

        ret
    }
}
