use std::collections::VecDeque;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut deque = VecDeque::new();
        let mut ret = i32::MIN;

        for j in 0..points.len() {
            let (xj, yj) = (points[j][0], points[j][1]);

            while xj - deque.front().unwrap_or(&(xj, 0)).0 > k {
                deque.pop_front();
            }

            if let Some(&(xi, yi)) = deque.front() {
                ret = ret.max(yi + yj + xj - xi);
            }

            while let Some(&(xi, yi)) = deque.back() {
                if yi - xi <= yj - xj {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back((xj, yj));
        }

        ret
    }
}
