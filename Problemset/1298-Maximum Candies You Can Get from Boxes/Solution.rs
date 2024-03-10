use std::collections::HashSet;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut closed = HashSet::new();
        let mut opened = vec![];
        let mut used = HashSet::new();
        let mut ret = 0;

        for b in initial_boxes {
            if status[b as usize] == 1 && !used.contains(&b) {
                opened.push(b);
                used.insert(b);
            } else if status[b as usize] == 0 {
                closed.insert(b);
            }
        }

        while let Some(b) = opened.pop() {
            ret += candies[b as usize];

            for &k in &keys[b as usize] {
                status[k as usize] = 1;
                if closed.remove(&k) && !used.contains(&k) {
                    opened.push(k);
                    used.insert(k);
                }
            }

            for &cb in &contained_boxes[b as usize] {
                if status[cb as usize] == 1 && !used.contains(&cb) {
                    opened.push(cb);
                    used.insert(cb);
                } else if status[cb as usize] == 0 {
                    closed.insert(cb);
                }
            }
        }

        ret
    }
}
