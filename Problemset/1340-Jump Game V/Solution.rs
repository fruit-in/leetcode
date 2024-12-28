impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let mut indices = (0..arr.len()).collect::<Vec<_>>();
        let mut max_visit = vec![1; arr.len()];

        indices.sort_unstable_by_key(|&i| arr[i]);

        for i in indices {
            let mut max = arr[i];

            for j in (0.max(i.saturating_sub(d))..i).rev() {
                if arr[j] > max {
                    max_visit[j] = max_visit[j].max(max_visit[i] + 1);
                    max = arr[j];
                }
            }

            max = arr[i];

            for j in i + 1..arr.len().min(i + d + 1) {
                if arr[j] > max {
                    max_visit[j] = max_visit[j].max(max_visit[i] + 1);
                    max = arr[j];
                }
            }
        }

        max_visit.into_iter().max().unwrap()
    }
}
