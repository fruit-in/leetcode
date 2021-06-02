# 15. 三数之和
给你一个包含 `n` 个整数的数组 `nums`，判断 `nums` 中是否存在三个元素 *a*，*b*，*c* ，使得 *a + b + c* = 0 ？请你找出所有和为 `0` 且不重复的三元组。

**注意:** 答案中不可以包含重复的三元组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-1,0,1,2,-1,-4]
<strong>输出:</strong> [[-1,-1,2],[-1,0,1]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = []
<strong>输出:</strong> []
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0]
<strong>输出:</strong> []
</pre>

#### 提示:
* `0 <= nums.length <= 3000`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut triplets = HashSet::new();
        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    triplets.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                } else if nums[i] + nums[j] + nums[k] < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        triplets.into_iter().collect()
    }
}
```
