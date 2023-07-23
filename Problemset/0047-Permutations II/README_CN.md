# 47. 全排列 II
给定一个可包含重复数字的序列，返回所有不重复的全排列。

#### 示例:
<pre>
<strong>输入:</strong> [1,1,2]
<strong>输出:</strong>
[
  [1,1,2],
  [1,2,1],
  [2,1,1]
]
</pre>

## 题解 (Rust)

### 1. 回溯
```Rust
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![Vec::new()];
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            if i == 0 || nums[i] != nums[i - 1] {
                let mut nums_clone = nums.clone();
                nums_clone.remove(i);

                let mut back_ret = Self::permute_unique(nums_clone);

                for arr in &mut back_ret {
                    arr.push(nums[i]);
                }
                ret.append(&mut back_ret);
            }
        }

        ret
    }
}
```
