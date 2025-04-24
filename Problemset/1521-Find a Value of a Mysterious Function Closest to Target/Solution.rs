impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let n = (*arr.iter().max().unwrap() as f64).log2().ceil() as usize;
        let mut prefix_zeros = vec![vec![0; n]; arr.len() + 1];
        let mut ret = i32::MAX;

        for i in 0..arr.len() {
            for j in 0..n {
                prefix_zeros[i + 1][j] = prefix_zeros[i][j] + 1 - ((arr[i] >> j) & 1);
            }
        }

        for r in 0..arr.len() {
            if arr[r] <= target {
                ret = ret.min(target - arr[r]);
                continue;
            }

            let mut x = 0;
            let mut lo = 0;
            let mut hi = r;

            while lo < hi {
                let mid = (lo + hi) / 2;

                x = (0..n)
                    .filter(|&i| prefix_zeros[r + 1][i] - prefix_zeros[mid][i] == 0)
                    .fold(0, |acc, i| acc | (1 << i));

                if x == target {
                    return 0;
                } else if x < target {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }

            x = (0..n)
                .filter(|&i| prefix_zeros[r + 1][i] - prefix_zeros[hi][i] == 0)
                .fold(0, |acc, i| acc | (1 << i));
            ret = ret.min((x - target).abs());

            if hi > 0 {
                x = (0..n)
                    .filter(|&i| prefix_zeros[r + 1][i] - prefix_zeros[hi - 1][i] == 0)
                    .fold(0, |acc, i| acc | (1 << i));
                ret = ret.min((x - target).abs());
            }
        }

        ret
    }
}
