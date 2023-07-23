impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut digits = (1..=pattern.len() + 1).rev().collect::<Vec<_>>();
        let mut stack = vec![];
        let mut num = vec![];

        for ch in pattern.chars().chain(std::iter::once('I')) {
            stack.push(digits.pop().unwrap() as u8 + b'0');

            while ch == 'I' && !stack.is_empty() {
                num.push(stack.pop().unwrap());
            }
        }

        String::from_utf8(num).unwrap()
    }
}
