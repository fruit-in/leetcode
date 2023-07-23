# 561. 数组拆分 I
给定长度为 2n 的数组, 你的任务是将这些数分成 n 对, 例如 (a<sub>1</sub>, b<sub>1</sub>), (a<sub>2</sub>, b<sub>2</sub>), ..., (a<sub>n</sub>, b<sub>n</sub>) ，使得从1 到 n 的 min(a<sub>i</sub>, b<sub>i</sub>) 总和最大。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,4,3,2]
<strong>输出:</strong> 4
<strong>解释:</strong> n 等于 2, 最大总和为 4 = min(1, 2) + min(3, 4).
</pre>

#### 提示:
1. **n** 是正整数,范围在 [1, 10000].
2. 数组中的元素范围在 [-10000, 10000].

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        nums.iter().step_by(2).sum()
    }
}
```
