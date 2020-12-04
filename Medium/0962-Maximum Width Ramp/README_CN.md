# 962. 最大宽度坡
给定一个整数数组 `A`，*坡*是元组 `(i, j)`，其中  `i < j` 且 `A[i] <= A[j]`。这样的坡的宽度为 `j - i`。

找出 `A` 中的坡的最大宽度，如果不存在，返回 0 。

#### 示例 1:
<pre>
<strong>输入:</strong> [6,0,8,2,1,5]
<strong>输出:</strong> 4
<strong>解释:</strong>
最大宽度的坡为 (i, j) = (1, 5): A[1] = 0 且 A[5] = 5.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [9,8,1,0,1,9,4,0,4,1]
<strong>输出:</strong> 7
<strong>解释:</strong>
最大宽度的坡为 (i, j) = (2, 9): A[2] = 1 且 A[9] = 1.
</pre>

#### 提示:
1. `2 <= A.length <= 50000`
2. `0 <= A[i] <= 50000`

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut v = a
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<Vec<_>>();
        let mut min_i = a.len();
        let mut ret = 0;

        v.sort_unstable();

        for (_, i) in v {
            ret = ret.max(i.saturating_sub(min_i));
            min_i = min_i.min(i);
        }

        ret as i32
    }
}
```
