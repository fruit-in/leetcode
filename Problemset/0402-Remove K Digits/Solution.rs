impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let keep = num.len() - k as usize;
        let mut stack = vec![];

        for (i, digit) in num.bytes().enumerate() {
            while *stack.last().unwrap_or(&b'0') > digit && num.len() - i + stack.len() > keep {
                stack.pop();
            }

            if stack.len() < keep {
                stack.push(digit);
            }
        }

        if stack.iter().all(|&digit| digit == b'0') {
            return "0".to_string();
        }

        String::from_utf8(stack)
            .unwrap()
            .trim_start_matches('0')
            .to_string()
    }
}
