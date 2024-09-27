use std::collections::HashMap;

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut children = vec![vec![]; parents.len()];
        let mut children_count = vec![0; parents.len()];
        let mut nodes = vec![];
        let mut size = vec![1_i64; parents.len()];
        let mut score_count = HashMap::new();

        for i in 1..parents.len() {
            children[parents[i] as usize].push(i);
            children_count[parents[i] as usize] += 1;
        }

        for i in 0..parents.len() {
            if children_count[i] == 0 {
                nodes.push(i);
            }
        }

        while let Some(i) = nodes.pop() {
            size[parents[i] as usize] += size[i];
            children_count[parents[i] as usize] -= 1;
            if parents[i] > 0 && children_count[parents[i] as usize] == 0 {
                nodes.push(parents[i] as usize);
            }
        }

        for i in 0..parents.len() {
            let mut score = 1;

            if i > 0 {
                score *= size[0] - size[i];
            }
            if let Some(&j) = children[i].get(0) {
                score *= size[j];
            }
            if let Some(&j) = children[i].get(1) {
                score *= size[j];
            }

            *score_count.entry(score).or_insert(0) += 1;
        }

        *score_count.iter().max().unwrap().1
    }
}
