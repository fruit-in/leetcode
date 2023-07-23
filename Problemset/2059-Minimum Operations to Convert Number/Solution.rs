use std::collections::VecDeque;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut unvisited = [true; 1001];
        let mut xs = VecDeque::from([(start, 0)]);

        while let Some((x, count)) = xs.pop_front() {
            for &num in &nums {
                let (a, b, c) = (x + num, x - num, x ^ num);

                if a == goal || b == goal || c == goal {
                    return count + 1;
                }

                if a >= 0 && a <= 1000 && unvisited[a as usize] {
                    unvisited[a as usize] = false;
                    xs.push_back((a, count + 1));
                }
                if b >= 0 && b <= 1000 && unvisited[b as usize] {
                    unvisited[b as usize] = false;
                    xs.push_back((b, count + 1));
                }
                if c >= 0 && c <= 1000 && unvisited[c as usize] {
                    unvisited[c as usize] = false;
                    xs.push_back((c, count + 1));
                }
            }
        }

        -1
    }
}
