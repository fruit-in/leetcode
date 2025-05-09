# 40. 组合总和 II
给定一个候选人编号的集合 `candidates` 和一个目标数 `target` ，找出 `candidates` 中所有可以使数字和为 `target` 的组合。

`candidates` 中的每个数字在每个组合中只能使用 **一次** 。

**注意：**解集不能包含重复的组合。

#### 示例 1:
<pre>
<strong>输入:</strong> candidates = [10,1,2,7,6,1,5], target = 8
<strong>输出:</strong>
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> candidates = [2,5,2,1,2], target = 5
<strong>输出:</strong>
[
[1,2,2],
[5]
]
</pre>

#### 提示:
* `1 <= candidates.length <= 100`
* `1 <= candidates[i] <= 50`
* `1 <= target <= 30`

## 题解 (Rust)

### 1. 题解
```Rust
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
```
