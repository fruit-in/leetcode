# 2154. 将找到的值乘以 2
给你一个整数数组 `nums` ，另给你一个整数 `original` ，这是需要在 `nums` 中搜索的第一个数字。

接下来，你需要按下述步骤操作：
1. 如果在 `nums` 中找到 `original` ，将 `original` **乘以** 2 ，得到新 `original`（即，令 `original = 2 * original`）。
2. 否则，停止这一过程。
3. 只要能在数组中找到新 `original` ，就对新 `original` 继续 **重复** 这一过程。

返回 `original` 的 **最终** 值。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,3,6,1,12], original = 3
<strong>输出:</strong> 24
<strong>解释:</strong>
- 3 能在 nums 中找到。3 * 2 = 6 。
- 6 能在 nums 中找到。6 * 2 = 12 。
- 12 能在 nums 中找到。12 * 2 = 24 。
- 24 不能在 nums 中找到。因此，返回 24 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,7,9], original = 4
<strong>输出:</strong> 4
<strong>解释:</strong>
- 4 不能在 nums 中找到。因此，返回 4 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= nums[i], original <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut original = original;
        let mut flag = 0;

        for num in nums {
            if num % original == 0 && (num / original).count_ones() == 1 {
                flag |= num / original;
            }
        }

        while flag & 1 == 1 {
            original *= 2;
            flag >>= 1;
        }

        original
    }
}
```
