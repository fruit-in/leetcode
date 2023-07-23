impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut start = vec![true; n as usize];

        for edge in edges {
            start[edge[1] as usize] = false;
        }

        (0..n).filter(|&x| start[x as usize]).collect()
    }
}
