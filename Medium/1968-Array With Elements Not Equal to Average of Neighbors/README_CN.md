# 1968. 构造元素不等于两相邻元素平均值的数组
给你一个 **下标从 0 开始** 的数组 `nums` ，数组由若干 **互不相同的** 整数组成。你打算重新排列数组中的元素以满足：重排后，数组中的每个元素都 **不等于** 其两侧相邻元素的 **平均值** 。

更公式化的说法是，重新排列的数组应当满足这一属性：对于范围 `1 <= i < nums.length - 1` 中的每个 `i` ，`(nums[i-1] + nums[i+1]) / 2` **不等于** `nums[i]` 均成立 。

返回满足题意的任一重排结果。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> [1,2,4,5,3]
<strong>解释:</strong>
i=1, nums[i] = 2, 两相邻元素平均值为 (1+4) / 2 = 2.5
i=2, nums[i] = 4, 两相邻元素平均值为 (2+5) / 2 = 3.5
i=3, nums[i] = 5, 两相邻元素平均值为 (4+3) / 2 = 3.5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [6,2,0,9,7]
<strong>输出:</strong> [9,7,6,2,0]
<strong>解释:</strong>
i=1, nums[i] = 7, 两相邻元素平均值为 (9+6) / 2 = 7.5
i=2, nums[i] = 6, 两相邻元素平均值为 (7+2) / 2 = 4.5
i=3, nums[i] = 2, 两相邻元素平均值为 (6+0) / 2 = 3
</pre>

#### 提示:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = vec![];
        nums.sort_unstable();

        if nums.len() % 2 == 1 {
            ret.push(nums.pop().unwrap());
        }

        for i in 0..nums.len() / 2 {
            ret.push(nums[i]);
            ret.push(nums[nums.len() / 2 + i]);
        }

        ret
    }
}
```
