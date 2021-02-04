use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadends = deadends
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<_>>();
        let target = target.parse::<i32>().unwrap();
        deadends.insert(target);
        let mut states = VecDeque::new();
        states.push_back((target, 0));

        while let Some((state, i)) = states.pop_front() {
            if state == 0 {
                return i;
            }

            for j in 0..4 {
                for k in &[1, 9] {
                    let mut new_state = [
                        state / 1000,
                        state % 1000 / 100,
                        state % 100 / 10,
                        state % 10,
                    ];
                    new_state[j] = (new_state[j] + k) % 10;
                    let new_state =
                        new_state[0] * 1000 + new_state[1] * 100 + new_state[2] * 10 + new_state[3];

                    if !deadends.contains(&new_state) {
                        deadends.insert(new_state);
                        states.push_back((new_state, i + 1));
                    }
                }
            }
        }

        -1
    }
}
