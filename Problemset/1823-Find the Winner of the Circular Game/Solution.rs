use std::collections::VecDeque;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = (1..=n).collect::<VecDeque<_>>();

        while friends.len() > 1 {
            friends.rotate_left((k as usize - 1) % friends.len());
            friends.pop_front();
        }

        friends[0]
    }
}
