impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0].contains(&edges[1][0]) {
            edges[1][0]
        } else {
            edges[1][1]
        }
    }
}
