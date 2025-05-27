impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut children = vec![vec![]; n];
        let mut parent = vec![n; n];
        let mut stack = vec![0];
        let mut indegree = vec![0; n];
        let mut subtree_nodes_count = vec![1; n];
        let mut answer = vec![0; n];

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            children[a].push(b);
            children[b].push(a);
            indegree[a] += 1;
            indegree[b] += 1;
        }

        while let Some(i) = stack.pop() {
            for &j in &children[i] {
                if j != 0 && parent[j] == n {
                    parent[j] = i;
                    stack.push(j);
                }
            }
        }

        for i in 1..n {
            indegree[i] -= 1;
            if children[i].len() == 1 {
                children[i].pop();
                stack.push(i);
                continue;
            }

            for j in 0..children[i].len() {
                if children[i][j] == parent[i] {
                    children[i].swap_remove(j);
                    break;
                }
            }
        }

        while let Some(i) = stack.pop() {
            if i == 0 {
                break;
            }

            indegree[parent[i]] -= 1;
            if indegree[parent[i]] == 0 {
                stack.push(parent[i]);
            }
            subtree_nodes_count[parent[i]] += subtree_nodes_count[i];
            answer[parent[i]] += answer[i] + subtree_nodes_count[i];
        }

        stack.push(0);
        while let Some(i) = stack.pop() {
            for &j in &children[i] {
                answer[j] = answer[i] - subtree_nodes_count[j] * 2 + n as i32;
                stack.push(j);
            }
        }

        answer
    }
}
