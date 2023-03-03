# 2295. 替换数组中的元素
给你一个下标从 **0** 开始的数组 `nums` ，它包含 `n` 个 **互不相同** 的正整数。请你对这个数组执行 `m` 个操作，在第 `i` 个操作中，你需要将数字 `operations[i][0]` 替换成 `operations[i][1]` 。

题目保证在第 `i` 个操作中：

* `operations[i][0]` 在 `nums` 中存在。
* `operations[i][1]` 在 `nums` 中不存在。

请你返回执行完所有操作后的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,4,6], operations = [[1,3],[4,7],[6,1]]
<strong>输出:</strong> [3,2,7,1]
<strong>解释:</strong> 我们对 nums 执行以下操作：
- 将数字 1 替换为 3 。nums 变为 [3,2,4,6] 。
- 将数字 4 替换为 7 。nums 变为 [3,2,7,6] 。
- 将数字 6 替换为 1 。nums 变为 [3,2,7,1] 。
返回最终数组 [3,2,7,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2], operations = [[1,3],[2,1],[3,2]]
<strong>输出:</strong> [2,1]
<strong>解释:</strong> 我们对 nums 执行以下操作：
- 将数字 1 替换为 3 。nums 变为 [3,2] 。
- 将数字 2 替换为 1 。nums 变为 [3,1] 。
- 将数字 3 替换为 2 。nums 变为 [2,1] 。
返回最终数组 [2,1] 。
</pre>

#### 提示:
* `n == nums.length`
* `m == operations.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* `nums` 中所有数字 **互不相同** 。
* `operations[i].length == 2`
* <code>1 <= nums[i], operations[i][0], operations[i][1] <= 10<sup>6</sup></code>
* 在执行第 `i` 个操作时，`operations[i][0]` 在 `nums` 中存在。
* 在执行第 `i` 个操作时，`operations[i][1]` 在 `nums` 中不存在。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut indices = nums
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect::<HashMap<_, _>>();

        for operation in operations {
            let i = indices.remove(&operation[0]).unwrap();
            nums[i] = operation[1];
            indices.insert(nums[i], i);
        }

        nums
    }
}
```
