impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n + 1];
        let mut group = vec![0; n + 1];

        for edge in &dislikes {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        for i in 1..=n {
            if group[i] > 0 {
                continue;
            }

            group[i] = 1;
            let mut stack = vec![i];

            while let Some(a) = stack.pop() {
                for &b in &neighbors[a] {
                    if group[b] == 0 {
                        group[b] = 3 - group[a];
                        stack.push(b);
                    } else if group[b] == group[a] {
                        return false;
                    }
                }
            }
        }

        true
    }
}
