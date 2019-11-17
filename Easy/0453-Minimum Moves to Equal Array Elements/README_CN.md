# 453. 最小移动次数使数组元素相等
给定一个长度为 *n* 的**非空**整数数组，找到让数组所有元素相等的最小移动次数。每次移动可以使 *n* - 1 个元素增加 1。

#### 示例:
<pre>
<strong>输入:</strong>
[1,2,3]
<strong>输出:</strong>
3
<strong>解释:</strong>
只需要3次移动（注意每次移动会增加两个元素的值）：

[1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
</pre>

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += (nums[i - 1] - nums[i]) * i as i32;
        }

        ret
    }
}
```

### 2. 数学
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min_num = nums.iter().min().unwrap();

        nums.iter().map(|&x| x - min_num).sum()
    }
}
```
