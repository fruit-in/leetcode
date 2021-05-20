impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut digits = [false; 10];

        for c in s.bytes() {
            if c.is_ascii_digit() {
                digits[(c - b'0') as usize] = true;
            }
        }

        (0..=9)
            .rev()
            .skip_while(|&x| !digits[x as usize])
            .skip(1)
            .find(|&x| digits[x as usize])
            .unwrap_or(-1)
    }
}
