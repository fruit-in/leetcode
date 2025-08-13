use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut deque = VecDeque::from([(source[0], source[1], 0)]);
        let mut visited = blocked.iter().map(|b| (b[0], b[1])).collect::<HashSet<_>>();
        visited.insert((source[0], source[1]));

        while let Some((x, y, steps)) = deque.pop_front() {
            if x == target[0] && y == target[1] {
                return true;
            }

            if steps >= blocked.len() * 2 {
                deque.push_back((0, 0, 0));
                break;
            }

            if x > 0 && !visited.contains(&(x - 1, y)) {
                deque.push_back((x - 1, y, steps + 1));
                visited.insert((x - 1, y));
            }
            if y > 0 && !visited.contains(&(x, y - 1)) {
                deque.push_back((x, y - 1, steps + 1));
                visited.insert((x, y - 1));
            }
            if x < 999999 && !visited.contains(&(x + 1, y)) {
                deque.push_back((x + 1, y, steps + 1));
                visited.insert((x + 1, y));
            }
            if y < 999999 && !visited.contains(&(x, y + 1)) {
                deque.push_back((x, y + 1, steps + 1));
                visited.insert((x, y + 1));
            }
        }

        if deque.is_empty() {
            return false;
        }

        deque = VecDeque::from([(target[0], target[1], 0)]);
        visited = blocked.iter().map(|b| (b[0], b[1])).collect::<HashSet<_>>();
        visited.insert((target[0], target[1]));

        while let Some((x, y, steps)) = deque.pop_front() {
            if steps >= blocked.len() * 2 {
                deque.push_back((0, 0, 0));
                break;
            }

            if x > 0 && !visited.contains(&(x - 1, y)) {
                deque.push_back((x - 1, y, steps + 1));
                visited.insert((x - 1, y));
            }
            if y > 0 && !visited.contains(&(x, y - 1)) {
                deque.push_back((x, y - 1, steps + 1));
                visited.insert((x, y - 1));
            }
            if x < 999999 && !visited.contains(&(x + 1, y)) {
                deque.push_back((x + 1, y, steps + 1));
                visited.insert((x + 1, y));
            }
            if y < 999999 && !visited.contains(&(x, y + 1)) {
                deque.push_back((x, y + 1, steps + 1));
                visited.insert((x, y + 1));
            }
        }

        !deque.is_empty()
    }
}
