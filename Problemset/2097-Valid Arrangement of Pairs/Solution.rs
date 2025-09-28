use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut outdegree = HashMap::new();
        let mut indegree = HashMap::new();
        let mut unused = HashMap::new();
        let mut stack = vec![];
        let mut curr = pairs[0][0];
        let mut path = vec![];
        let mut ret = vec![];

        for pair in &pairs {
            *outdegree.entry(pair[0]).or_insert(0) += 1;
            *indegree.entry(pair[1]).or_insert(0) += 1;
            unused.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        }

        for pair in &pairs {
            if outdegree[&pair[0]] == *indegree.get(&pair[0]).unwrap_or(&0) + 1 {
                curr = pair[0];
                break;
            }
        }

        while curr != -1 {
            if !unused.get(&curr).unwrap_or(&vec![]).is_empty() {
                stack.push(curr);
                curr = unused.get_mut(&curr).unwrap().pop().unwrap();
            } else {
                path.push(curr);
                curr = stack.pop().unwrap_or(-1);
            }
        }

        for i in (1..path.len()).rev() {
            ret.push(vec![path[i], path[i - 1]]);
        }

        ret
    }
}
