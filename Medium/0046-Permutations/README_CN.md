# 46. 全排列
给定一个 **没有重复** 数字的序列，返回其所有可能的全排列。

#### 示例:
<pre>
<strong>输入:</strong> [1,2,3]
<strong>输出:</strong>
[
  [1,2,3],
  [1,3,2],
  [2,1,3],
  [2,3,1],
  [3,1,2],
  [3,2,1]
]
</pre>

## 题解 (Rust)

### 1. 回溯
```Rust
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![Vec::new()];
        }

        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let mut nums_clone = nums.clone();
            nums_clone.remove(i);

            let mut back_ret = Self::permute(nums_clone);

            for arr in &mut back_ret {
                arr.push(nums[i]);
            }
            ret.append(&mut back_ret);
        }

        ret
    }
}
```
