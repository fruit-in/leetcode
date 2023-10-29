use std::collections::HashSet;

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        let mut stack = vec![];
        let mut used = HashSet::new();

        trees.sort_unstable();
        stack.push(trees[0].clone());
        used.insert(trees[0].clone());

        for i in 1..trees.len() {
            let (x0, y0) = (trees[i][0], trees[i][1]);

            while stack.len() > 1 {
                let (x1, y1) = (stack[stack.len() - 1][0], stack[stack.len() - 1][1]);
                let (x2, y2) = (stack[stack.len() - 2][0], stack[stack.len() - 2][1]);

                if (x1 - x2) * (y0 - y1) < (x0 - x1) * (y1 - y2) {
                    used.remove(&vec![x1, y1]);
                    stack.pop();
                } else {
                    break;
                }
            }

            used.insert(vec![x0, y0]);
            stack.push(vec![x0, y0]);
        }
        for i in (0..trees.len() - 1).rev() {
            let (x0, y0) = (trees[i][0], trees[i][1]);

            if i > 0 && used.contains(&vec![x0, y0]) {
                continue;
            }

            while stack.len() > 1 {
                let (x1, y1) = (stack[stack.len() - 1][0], stack[stack.len() - 1][1]);
                let (x2, y2) = (stack[stack.len() - 2][0], stack[stack.len() - 2][1]);

                if (x1 - x2) * (y0 - y1) < (x0 - x1) * (y1 - y2) {
                    used.remove(&vec![x1, y1]);
                    stack.pop();
                } else {
                    break;
                }
            }

            used.insert(vec![x0, y0]);
            stack.push(vec![x0, y0]);
        }

        used.into_iter().collect()
    }
}
