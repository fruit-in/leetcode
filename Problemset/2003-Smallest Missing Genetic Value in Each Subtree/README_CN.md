# 2003. 每棵子树内缺失的最小基因值
有一棵根节点为 `0` 的 **家族树** ，总共包含 `n` 个节点，节点编号为 `0` 到 `n - 1` 。给你一个下标从 **0** 开始的整数数组 `parents` ，其中 `parents[i]` 是节点 `i` 的父节点。由于节点 `0` 是 **根** ，所以 `parents[0] == -1` 。

总共有 <code>10<sup>5</sup></code> 个基因值，每个基因值都用 **闭区间** <code>[1, 10<sup>5</sup>]</code> 中的一个整数表示。给你一个下标从 **0** 开始的整数数组 `nums` ，其中 `nums[i]` 是节点 `i` 的基因值，且基因值 **互不相同** 。

请你返回一个数组 `ans` ，长度为 `n` ，其中 `ans[i]` 是以节点 `i` 为根的子树内 **缺失** 的 **最小** 基因值。

节点 `x` 为根的 **子树** 包含节点 `x` 和它所有的 **后代** 节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/23/case-1.png)
<pre>
<strong>输入:</strong> parents = [-1,0,0,2], nums = [1,2,3,4]
<strong>输出:</strong> [5,1,1,1]
<strong>解释:</strong> 每个子树答案计算结果如下：
- 0：子树包含节点 [0,1,2,3] ，基因值分别为 [1,2,3,4] 。5 是缺失的最小基因值。
- 1：子树只包含节点 1 ，基因值为 2 。1 是缺失的最小基因值。
- 2：子树包含节点 [2,3] ，基因值分别为 [3,4] 。1 是缺失的最小基因值。
- 3：子树只包含节点 3 ，基因值为 4 。1是缺失的最小基因值。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/08/23/case-2.png)
<pre>
<strong>输入:</strong> parents = [-1,0,1,0,3,3], nums = [5,4,6,2,1,3]
<strong>输出:</strong> [7,1,1,4,2,1]
<strong>解释:</strong> 每个子树答案计算结果如下：
- 0：子树内包含节点 [0,1,2,3,4,5] ，基因值分别为 [5,4,6,2,1,3] 。7 是缺失的最小基因值。
- 1：子树内包含节点 [1,2] ，基因值分别为 [4,6] 。 1 是缺失的最小基因值。
- 2：子树内只包含节点 2 ，基因值为 6 。1 是缺失的最小基因值。
- 3：子树内包含节点 [3,4,5] ，基因值分别为 [2,1,3] 。4 是缺失的最小基因值。
- 4：子树内只包含节点 4 ，基因值为 1 。2 是缺失的最小基因值。
- 5：子树内只包含节点 5 ，基因值为 3 。1 是缺失的最小基因值。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> parents = [-1,2,3,0,2,4,1], nums = [2,3,4,5,6,7,8]
<strong>输出:</strong> [1,1,1,1,1,1,1]
<strong>解释:</strong> 所有子树都缺失基因值 1 。
</pre>

#### 提示:
* `n == parents.length == nums.length`
* <code>2 <= n <= 105</sup></code>
* 对于 `i != 0` ，满足 `0 <= parents[i] <= n - 1`
* `parents[0] == -1`
* `parents` 表示一棵合法的树。
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* `nums[i]` 互不相同。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut children = vec![vec![]; n];
        let mut sub_genetic = HashSet::new();
        let mut min_val = 1;
        let mut ans = vec![1; n];

        for i in 1..n {
            children[parents[i] as usize].push(i);
        }

        for i in 0..n {
            if nums[i] > 1 {
                continue;
            }

            let mut j = i as i32;

            while j != -1 {
                let mut stack = vec![j as usize];

                while let Some(k) = stack.pop() {
                    if !sub_genetic.contains(&nums[k]) {
                        sub_genetic.insert(nums[k]);
                        for &l in &children[k] {
                            stack.push(l);
                        }
                    }
                }

                while sub_genetic.contains(&min_val) {
                    min_val += 1;
                }
                ans[j as usize] = min_val;

                j = parents[j as usize];
            }

            break;
        }

        ans
    }
}
```
