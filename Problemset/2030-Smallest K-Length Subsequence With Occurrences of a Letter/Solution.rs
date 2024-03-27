impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let k = k as usize;
        let repetition = repetition as usize;
        let mut remain = s.chars().filter(|&c| c == letter).count();
        let mut count = 0;
        let mut stack = vec![];

        for (i, c) in s.chars().enumerate() {
            while *stack.last().unwrap_or(&'a') > c && s.len() - i + stack.len() > k {
                if *stack.last().unwrap() == letter {
                    if count + remain > repetition {
                        count -= 1;
                    } else {
                        break;
                    }
                }

                stack.pop();
            }

            if c == letter && count + remain == repetition && stack.len() + remain >= k {
                while stack.len() + remain > k {
                    remain += (stack.pop().unwrap() == letter) as usize;
                }

                stack.append(&mut vec![letter; remain]);

                break;
            }

            if stack.len() < k {
                count += (c == letter) as usize;
                stack.push(c);
            }

            remain -= (c == letter) as usize;
        }

        stack.into_iter().collect()
    }
}
