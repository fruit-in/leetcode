# 1793. 好子数组的最大分数
给你一个整数数组 `nums` **（下标从 0 开始）**和一个整数 `k` 。

一个子数组 `(i, j)` 的 **分数** 定义为 `min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1)` 。一个 **好** 子数组的两个端点下标需要满足 `i <= k <= j` 。

请你返回 **好** 子数组的最大可能 **分数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,4,3,7,4,5], k = 3
<strong>输出:</strong> 15
<strong>解释:</strong> 最优子数组的左右端点下标是 (1, 5) ，分数为 min(4,3,7,4,5) * (5-1+1) = 3 * 5 = 15 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,5,4,5,4,1,1,1], k = 0
<strong>输出:</strong> 20
<strong>解释:</strong> 最优子数组的左右端点下标是 (0, 4) ，分数为 min(5,5,4,5,4) * (4-0+1) = 4 * 5 = 20 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 2 * 10<sup>4</sup></code>
* `0 <= k < nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut i = k;
        let mut j = k;
        let mut min_num = nums[k as usize];
        let mut ret = min_num;

        loop {
            while i - 1 >= 0 && nums[i as usize - 1] >= min_num {
                i -= 1;
            }
            while j + 1 < nums.len() as i32 && nums[j as usize + 1] >= min_num {
                j += 1;
            }

            ret = ret.max(min_num * (j - i + 1));
            i -= 1;
            if i < 0 {
                break;
            }
            min_num = nums[i as usize];
        }

        i = k;
        j = k;
        min_num = nums[k as usize];

        loop {
            while i - 1 >= 0 && nums[i as usize - 1] >= min_num {
                i -= 1;
            }
            while j + 1 < nums.len() as i32 && nums[j as usize + 1] >= min_num {
                j += 1;
            }

            ret = ret.max(min_num * (j - i + 1));
            j += 1;
            if j >= nums.len() as i32 {
                break;
            }
            min_num = nums[j as usize];
        }

        ret
    }
}
```
