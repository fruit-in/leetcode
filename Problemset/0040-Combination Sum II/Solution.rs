impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates
            .into_iter()
            .map(|x| x as usize)
            .collect::<Vec<_>>();
        let target = target as usize;
        let mut nums = vec![];
        let mut sums = vec![vec![]; target + 1];

        candidates.sort_unstable();
        sums[0].push(vec![]);

        for &x in &candidates {
            if x > target {
                break;
            } else if nums.last().unwrap_or(&(0, 0)).0 != x {
                nums.push((x, 1));
            } else if (nums.last().unwrap().1 + 1) * x <= target {
                nums.last_mut().unwrap().1 += 1;
            }
        }

        for &(x, c) in &nums {
            for i in (0..target).rev() {
                for mut v in sums[i].clone() {
                    for j in (i + x..=target.min(i + x * c)).step_by(x) {
                        v.push(x as i32);
                        sums[j].push(v.clone());
                    }
                }
            }
        }

        sums[target as usize].clone()
    }
}
