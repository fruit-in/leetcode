impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        let mut s = s.chars().peekable();
        while let Some(ch) = s.next() {
            match ch {
                'I' => match s.peek() {
                    Some('V') | Some('X') => n -= 1,
                    _ => n += 1,
                },
                'X' => match s.peek() {
                    Some('L') | Some('C') => n -= 10,
                    _ => n += 10,
                },
                'C' => match s.peek() {
                    Some('D') | Some('M') => n -= 100,
                    _ => n += 100,
                },
                'V' => n += 5,
                'L' => n += 50,
                'D' => n += 500,
                'M' => n += 1000,
                _ => (),
            };
        }
        n
    }
}
