# 60. 排列序列
给出集合 `[1,2,3,...,n]`，其所有元素共有 `n!` 种排列。

按大小顺序列出所有排列情况，并一一标记，当 `n = 3` 时, 所有排列如下：

1. `"123"`
2. `"132"`
3. `"213"`
4. `"231"`
5. `"312"`
6. `"321"`

给定 `n` 和 `k`，返回第 `k` 个排列。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, k = 3
<strong>输出:</strong> "213"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4, k = 9
<strong>输出:</strong> "2314"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3, k = 1
<strong>输出:</strong> "123"
</pre>

#### 提示:
* `1 <= n <= 9`
* `1 <= k <= n!`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize - 1;
        let mut digits = (b'1'..=(n as u8 + b'0')).collect::<Vec<_>>();
        let mut factorials = (0..n).collect::<Vec<_>>();
        let mut ret = vec![b'0'; n];

        factorials[0] = 1;
        for i in 3..n {
            factorials[i] *= factorials[i - 1];
        }

        for i in 0..n {
            ret[i] = digits.remove(k / factorials[n - i - 1]);
            k %= factorials[n - i - 1];
        }

        String::from_utf8(ret).unwrap()
    }
}
```
