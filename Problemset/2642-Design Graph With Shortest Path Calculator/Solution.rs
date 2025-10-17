use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Graph {
    tos: Vec<Vec<(usize, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut tos = vec![vec![]; n as usize];

        for edge in &edges {
            tos[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        Self { tos }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.tos[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let node1 = node1 as usize;
        let node2 = node2 as usize;
        let mut heap = BinaryHeap::from([(Reverse(0), node1)]);
        let mut min_cost = vec![i32::MAX; self.tos.len()];
        min_cost[node1] = 0;

        while let Some((Reverse(total_cost), from)) = heap.pop() {
            if from == node2 {
                return total_cost;
            } else if total_cost > min_cost[from] {
                continue;
            }

            for &(to, cost) in &self.tos[from] {
                if total_cost + cost < min_cost[to] {
                    min_cost[to] = total_cost + cost;
                    heap.push((Reverse(min_cost[to]), to));
                }
            }
        }

        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */
