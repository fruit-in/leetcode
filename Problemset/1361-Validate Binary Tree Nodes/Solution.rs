use std::collections::HashSet;

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut candidates = (0..n).collect::<HashSet<_>>();
        let mut stack = vec![];
        let mut visited = HashSet::new();

        for i in 0..left_child.len() {
            if left_child[i] >= 0 && !candidates.remove(&left_child[i]) {
                return false;
            }
            if right_child[i] >= 0 && !candidates.remove(&right_child[i]) {
                return false;
            }
        }

        if candidates.len() != 1 {
            return false;
        }

        stack.push(candidates.into_iter().next().unwrap() as usize);
        visited.insert(stack[0]);

        while let Some(i) = stack.pop() {
            if left_child[i] >= 0 {
                if visited.insert(left_child[i] as usize) {
                    stack.push(left_child[i] as usize);
                } else {
                    return false;
                }
            }
            if right_child[i] >= 0 {
                if visited.insert(right_child[i] as usize) {
                    stack.push(right_child[i] as usize);
                } else {
                    return false;
                }
            }
        }

        visited.len() == n as usize
    }
}
