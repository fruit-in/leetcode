# 442. 数组中重复的数据
给定一个整数数组 a，其中1 ≤ a[i] ≤ *n* （*n*为数组长度）, 其中有些元素出现**两次**而其他元素出现**一次**。

找到所有出现**两次**的元素。

你可以不用到任何额外空间并在O(*n*)时间复杂度内解决这个问题吗？

#### 示例:
<pre>
<strong>输入:</strong>
[4,3,2,7,8,2,3,1]
<strong>输出:</strong>
[2,3]
</pre>

## 题解 (Rust)

### 1. 标记位置
```Rust
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            nums[j] = -nums[j];
            if nums[j] > 0 {
                ret.push(j as i32 + 1);
            }
        }

        ret
    }
}
```
