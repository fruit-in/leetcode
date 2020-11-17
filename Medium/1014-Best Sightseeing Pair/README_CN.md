# 1014. 最佳观光组合
给定正整数数组 `A`，`A[i]` 表示第 `i` 个观光景点的评分，并且两个景点 `i` 和 `j` 之间的距离为 `j - i`。

一对景点（`i < j`）组成的观光组合的得分为（`A[i] + A[j] + i - j`）：景点的评分之和**减去**它们两者之间的距离。

返回一对观光景点能取得的最高分。

#### 示例:
<pre>
<b>输入:</b> [8,1,5,2,6]
<b>输出:</b> 11
<b>解释:</b> i = 0, j = 2, A[i] + A[j] + i - j = 8 + 5 + 0 - 2 = 11
</pre>

#### 提示:
1. `2 <= A.length <= 50000`
2. `1 <= A[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let mut score = a[0];
        let mut ret = 0;

        for j in 1..a.len() {
            ret = ret.max(score + a[j] - j as i32);
            score = score.max(a[j] + j as i32);
        }

        ret
    }
}
```
