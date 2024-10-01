use std::collections::HashSet;

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut edges_set = HashSet::new();
        let mut count = vec![0; n + 1];
        let mut ret = -1;

        for i in 0..edges.len() {
            let (u, v) = (
                edges[i][0].min(edges[i][1]) as usize,
                edges[i][0].max(edges[i][1]) as usize,
            );

            edges_set.insert((u, v));
            count[u] += 1;
            count[v] += 1;
        }

        for i in 1..=n {
            for j in i + 1..=n {
                if !edges_set.contains(&(i, j)) {
                    continue;
                }

                for k in j + 1..=n {
                    if !edges_set.contains(&(i, k)) || !edges_set.contains(&(j, k)) {
                        continue;
                    }

                    let degree = count[i] + count[j] + count[k] - 6;

                    if ret == -1 || degree < ret {
                        ret = degree;
                    }
                }
            }
        }

        ret
    }
}
