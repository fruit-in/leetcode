impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut scores = vec![0; edges.len()];

        for i in 0..edges.len() {
            scores[edges[i] as usize] += i as i64;
        }

        (0..edges.len())
            .max_by_key(|&i| (scores[i], -(i as i32)))
            .unwrap_or(0) as i32
    }
}
