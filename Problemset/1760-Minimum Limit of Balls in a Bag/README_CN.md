# 1760. 袋子里最少数目的球
给你一个整数数组 `nums` ，其中 `nums[i]` 表示第 `i` 个袋子里球的数目。同时给你一个整数 `maxOperations` 。

你可以进行如下操作至多 `maxOperations` 次：

* 选择任意一个袋子，并将袋子里的球分到 2 个新的袋子中，每个袋子里都有 **正整数** 个球。
    * 比方说，一个袋子里有 `5` 个球，你可以把它们分到两个新袋子里，分别有 `1` 个和 `4` 个球，或者分别有 `2` 个和 `3` 个球。

你的开销是单个袋子里球数目的 **最大值** ，你想要 **最小化** 开销。

请你返回进行上述操作后的最小开销。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [9], maxOperations = 2
<strong>输出:</strong> 3
<strong>解释:</strong>
- 将装有 9 个球的袋子分成装有 6 个和 3 个球的袋子。[9] -> [6,3] 。
- 将装有 6 个球的袋子分成装有 3 个和 3 个球的袋子。[6,3] -> [3,3,3] 。
装有最多球的袋子里装有 3 个球，所以开销为 3 并返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,4,8,2], maxOperations = 4
<strong>输出:</strong> 2
<strong>解释:</strong>
- 将装有 8 个球的袋子分成装有 4 个和 4 个球的袋子。[2,4,8,2] -> [2,4,4,4,2] 。
- 将装有 4 个球的袋子分成装有 2 个和 2 个球的袋子。[2,4,4,4,2] -> [2,2,2,4,4,2] 。
- 将装有 4 个球的袋子分成装有 2 个和 2 个球的袋子。[2,2,2,4,4,2] -> [2,2,2,2,2,4,2] 。
- 将装有 4 个球的袋子分成装有 2 个和 2 个球的袋子。[2,2,2,2,2,4,2] -> [2,2,2,2,2,2,2,2] 。
装有最多球的袋子里装有 2 个球，所以开销为 2 并返回 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [7,17], maxOperations = 2
<strong>输出:</strong> 7
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= maxOperations, nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut low = 1;
        let mut high = *nums.iter().max().unwrap();

        while low < high {
            let mid = (low + high) / 2;
            let mut operations = 0;

            for x in &nums {
                operations += (x - 1) / mid;
            }

            if operations > max_operations {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        high
    }
}
```
