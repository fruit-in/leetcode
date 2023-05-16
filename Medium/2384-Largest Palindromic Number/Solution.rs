impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut count = [0; 10];
        let mut digits = vec![];

        for digit in num.bytes() {
            count[(digit - b'0') as usize] += 1;
        }

        for i in (0..=9).rev() {
            while count[i] > 1 {
                count[i] -= 2;
                digits.push(i as u8 + b'0');
            }
        }

        if *digits.get(0).unwrap_or(&b'1') == b'0' {
            digits.clear();
        }

        let mut digits_rev = digits.clone();
        digits_rev.reverse();

        if let Some(i) = (0..=9).rev().find(|&i| count[i] > 0) {
            digits.push(i as u8 + b'0');
        }
        digits.append(&mut digits_rev);
        if digits.is_empty() {
            digits.push(b'0');
        }

        String::from_utf8(digits).unwrap()
    }
}
