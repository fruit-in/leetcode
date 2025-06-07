impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut count = vec![0; n as usize];

        for i in 0..roads.len() {
            count[roads[i][0] as usize] += 1;
            count[roads[i][1] as usize] += 1;
        }

        count.sort_unstable();

        count.iter().zip(1..=n as i64).map(|(c, v)| c * v).sum()
    }
}
