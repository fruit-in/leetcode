impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times = (0..dist.len())
            .map(|i| (dist[i] + speed[i] - 1) / speed[i])
            .collect::<Vec<_>>();

        times.sort_unstable();

        times
            .iter()
            .enumerate()
            .find(|&(i, &t)| i >= t as usize)
            .unwrap_or((times.len(), &0))
            .0 as i32
    }
}
