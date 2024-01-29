use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let forbidden = forbidden.into_iter().collect::<HashSet<_>>();
        let mut queue = VecDeque::from([(0, 0, true)]);
        let mut visited = HashSet::from([(0, true), (0, false)]);

        while let Some((pos, jumps, isforward)) = queue.pop_front() {
            if pos == x {
                return jumps;
            }

            if pos + a <= 10000
                && !forbidden.contains(&(pos + a))
                && visited.insert((pos + a, true))
            {
                queue.push_back((pos + a, jumps + 1, true));
            }
            if isforward
                && pos - b >= 0
                && !forbidden.contains(&(pos - b))
                && visited.insert((pos - b, false))
            {
                queue.push_back((pos - b, jumps + 1, false));
            }
        }

        -1
    }
}
