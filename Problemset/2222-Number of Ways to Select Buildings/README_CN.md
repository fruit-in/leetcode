# 2222. 选择建筑的方案数
给你一个下标从 **0** 开始的二进制字符串 `s` ，它表示一条街沿途的建筑类型，其中：

* `s[i] = '0'` 表示第 `i` 栋建筑是一栋办公楼，
* `s[i] = '1'` 表示第 `i` 栋建筑是一间餐厅。

作为市政厅的官员，你需要随机 **选择** 3 栋建筑。然而，为了确保多样性，选出来的 3 栋建筑 **相邻** 的两栋不能是同一类型。

* 比方说，给你 `s = "001101"` ，我们不能选择第 `1` ，`3` 和 `5` 栋建筑，因为得到的子序列是 `"011"` ，有相邻两栋建筑是同一类型，所以 **不合** 题意。

请你返回可以选择 3 栋建筑的 **有效方案数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "001101"
<strong>输出:</strong> 6
<strong>解释:</strong>
以下下标集合是合法的：
- [0,2,4] ，从 "001101" 得到 "010"
- [0,3,4] ，从 "001101" 得到 "010"
- [1,2,4] ，从 "001101" 得到 "010"
- [1,3,4] ，从 "001101" 得到 "010"
- [2,4,5] ，从 "001101" 得到 "101"
- [3,4,5] ，从 "001101" 得到 "101"
没有别的合法选择，所以总共有 6 种方法。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "11100"
<strong>输出:</strong> 0
<strong>解释:</strong> 没有任何符合题意的选择。
</pre>

#### 提示:
* <code>3 <= s.length <= 10<sup>5</sup></code>
* `s[i]` 要么是 `'0'` ，要么是 `'1'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut count0 = vec![0; s.len()];
        let mut count1 = vec![0; s.len()];

        for (i, building) in s.chars().enumerate() {
            if i > 0 {
                count0[i] = count0[i - 1];
                count1[i] = count1[i - 1];
            }

            count0[i] += (building == '0') as i64;
            count1[i] += (building == '1') as i64;
        }

        s.chars()
            .enumerate()
            .map(|(i, building)| match building {
                '0' => count1[i] * (count1[s.len() - 1] - count1[i]),
                _ => count0[i] * (count0[s.len() - 1] - count0[i]),
            })
            .sum()
    }
}
```
