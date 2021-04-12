use std::collections::HashSet;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut seen = HashSet::new();
        let mut a = HashSet::new();
        let mut b = HashSet::new();

        for i in 0..graph.len() {
            if !seen.contains(&i) {
                let mut insert_a = true;
                let mut nodes0 = vec![i];

                while !nodes0.is_empty() {
                    let mut nodes1 = vec![];

                    for j in nodes0 {
                        seen.insert(j);
                        if insert_a {
                            a.insert(j);
                        } else {
                            b.insert(j);
                        }

                        for &k in &graph[j] {
                            if !seen.contains(&(k as usize)) {
                                nodes1.push(k as usize);
                            }
                        }
                    }

                    insert_a = !insert_a;
                    nodes0 = nodes1;
                }
            }
        }

        a.intersection(&b).count() == 0
    }
}
