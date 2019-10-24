impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = vec![0; n as usize];
        let mut graph = vec![Vec::new(); n as usize];

        for path in paths {
            graph[path[0] as usize - 1].push(path[1] as usize - 1);
            graph[path[1] as usize - 1].push(path[0] as usize - 1);
        }

        for i in 0..answer.len() {
            let mut choice = 0;
            for &neighbor in &graph[i] {
                if neighbor < i {
                    choice |= 1 << (answer[neighbor] - 1);
                }
            }

            match choice {
                7 => answer[i] = 4,
                3|11 => answer[i] = 3,
                1|5|9|13 => answer[i] = 2,
                _ => answer[i] = 1,
            };
        }

        answer
    }
}
