# 462. 最少移动次数使数组元素相等 II
给定一个非空整数数组，找到使所有数组元素相等所需的最小移动数，其中每次移动可将选定的一个元素加1或减1。 您可以假设数组的长度最多为10000。

#### 例如:
<pre>
<strong>输入:</strong>
[1,2,3]
<strong>输出:</strong>
2
<strong>说明:</strong>
只有两个动作是必要的（记得每一步仅可使其中一个元素加1或减1）：

[1,2,3]  =>  [2,2,3]  =>  [2,2,2]
</pre>

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mid = nums[nums.len() / 2];

        nums.into_iter().map(|x| (x - mid).abs()).sum()
    }
}
```
