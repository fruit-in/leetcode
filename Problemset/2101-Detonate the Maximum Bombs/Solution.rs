use std::collections::HashSet;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut edges = vec![vec![]; bombs.len()];
        let mut visited = HashSet::new();
        let mut stack = vec![];
        let mut ret = 0;

        for i in 0..bombs.len() {
            for j in 0..bombs.len() {
                let x2 = ((bombs[i][0] - bombs[j][0]) as i64).pow(2);
                let y2 = ((bombs[i][1] - bombs[j][1]) as i64).pow(2);
                let r2 = (bombs[i][2] as i64).pow(2);

                if x2 + y2 <= r2 {
                    edges[i].push(j);
                }
            }
        }

        for i in 0..bombs.len() {
            visited.clear();
            visited.insert(i);
            stack.clear();
            stack.push(i);

            while let Some(j) = stack.pop() {
                for &k in &edges[j] {
                    if !visited.contains(&k) {
                        visited.insert(k);
                        stack.push(k);
                    }
                }
            }

            ret = ret.max(visited.len());
        }

        ret as i32
    }
}
