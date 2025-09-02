impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n as usize + 1];
        let mut stack = vec![1];
        let mut visited = vec![false; n as usize + 1];
        visited[1] = true;
        let mut ret = i32::MAX;

        for road in &roads {
            let (a, b, distance) = (road[0] as usize, road[1] as usize, road[2]);
            neighbors[a].push((b, distance));
            neighbors[b].push((a, distance));
        }

        while let Some(a) = stack.pop() {
            for &(b, distance) in &neighbors[a] {
                if !visited[b] {
                    stack.push(b);
                    visited[b] = true;
                }
                ret = ret.min(distance);
            }
        }

        ret
    }
}
