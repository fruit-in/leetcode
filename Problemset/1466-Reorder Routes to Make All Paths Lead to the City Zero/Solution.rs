impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut from_to = vec![vec![]; n];
        let mut to_from = vec![vec![]; n];
        let mut stack = vec![0];
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut ret = 0;

        for connection in &connections {
            let (a, b) = (connection[0] as usize, connection[1] as usize);
            from_to[a].push(b);
            to_from[b].push(a);
        }

        while let Some(a) = stack.pop() {
            for &b in &from_to[a] {
                if !visited[b] {
                    stack.push(b);
                    visited[b] = true;
                    ret += 1;
                }
            }
            for &b in &to_from[a] {
                if !visited[b] {
                    stack.push(b);
                    visited[b] = true;
                }
            }
        }

        ret
    }
}
