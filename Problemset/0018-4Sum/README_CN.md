# 18. 四数之和
给你一个由 `n` 个整数组成的数组 `nums` ，和一个目标值 `target` 。请你找出并返回满足下述全部条件且**不重复**的四元组 `[nums[a], nums[b], nums[c], nums[d]]` （若两个四元组元素一一对应，则认为两个四元组重复）：

* `0 <= a, b, c, d < n`
* `a`、`b`、`c` 和 `d` **互不相同**
* `nums[a] + nums[b] + nums[c] + nums[d] == target`

你可以按 **任意顺序** 返回答案 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,0,-1,0,-2,2], target = 0
<strong>输出:</strong> [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2,2,2], target = 8
<strong>输出:</strong> [[2,2,2,2]]
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* <code>-10<sup>9</sup> <= target <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut ret = HashSet::new();

        nums.sort_unstable();

        for a in 0..nums.len() {
            for b in a + 1..nums.len() {
                let mut c = b + 1;
                let mut d = nums.len() - 1;

                while c < d {
                    let sum = nums[a] + nums[b] + nums[c] + nums[d];

                    if sum == target {
                        let mut quadruplets = vec![
                            nums[a] as i32,
                            nums[b] as i32,
                            nums[c] as i32,
                            nums[d] as i32,
                        ];
                        quadruplets.sort_unstable();
                        ret.insert(quadruplets);
                        c += 1;
                        d -= 1;
                    } else if sum < target {
                        c += 1;
                    } else {
                        d -= 1;
                    }
                }
            }
        }

        ret.into_iter().collect()
    }
}
```
