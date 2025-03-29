# 2545. 根据第 K 场考试的分数排序
班里有 `m` 位学生，共计划组织 `n` 场考试。给你一个下标从 **0** 开始、大小为 `m x n` 的整数矩阵 `score` ，其中每一行对应一位学生，而 `score[i][j]` 表示第 `i` 位学生在第 `j` 场考试取得的分数。矩阵 `score` 包含的整数 **互不相同** 。

另给你一个整数 `k` 。请你按第 `k` 场考试分数从高到低完成对这些学生（矩阵中的行）的排序。

返回排序后的矩阵。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/11/30/example1.png)
<pre>
<strong>输入:</strong> score = [[10,6,9,1],[7,5,11,2],[4,8,3,15]], k = 2
<strong>输出:</strong> [[7,5,11,2],[10,6,9,1],[4,8,3,15]]
<strong>解释:</strong> 在上图中，S 表示学生，E 表示考试。
- 下标为 1 的学生在第 2 场考试取得的分数为 11 ，这是考试的最高分，所以 TA 需要排在第一。
- 下标为 0 的学生在第 2 场考试取得的分数为 9 ，这是考试的第二高分，所以 TA 需要排在第二。
- 下标为 2 的学生在第 2 场考试取得的分数为 3 ，这是考试的最低分，所以 TA 需要排在第三。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/11/30/example2.png)
<pre>
<strong>输入:</strong> score = [[3,4],[5,6]], k = 0
<strong>输出:</strong> [[5,6],[3,4]]
<strong>解释:</strong> 在上图中，S 表示学生，E 表示考试。
- 下标为 1 的学生在第 0 场考试取得的分数为 5 ，这是考试的最高分，所以 TA 需要排在第一。
- 下标为 0 的学生在第 0 场考试取得的分数为 3 ，这是考试的最低分，所以 TA 需要排在第二。
</pre>

#### 提示:
* `m == score.length`
* `n == score[i].length`
* `1 <= m, n <= 250`
* <code>1 <= score[i][j] <= 10<sup>5</sup></code>
* `score` 由 **不同** 的整数组成
* `0 <= k < n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_unstable_by_key(|row| -row[k as usize]);

        score
    }
}
```
