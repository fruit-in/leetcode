# 89. 格雷编码
格雷编码是一个二进制数字系统，在该系统中，两个连续的数值仅有一个位数的差异。

给定一个代表编码总位数的非负整数 *n*，打印其格雷编码序列。格雷编码序列必须以 0 开头。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> [0,1,3,2]
<strong>解释:</strong>
00 - 0
01 - 1
11 - 3
10 - 2

对于给定的 <em>n</em>，其格雷编码序列并不唯一。
例如，[0,2,3,1] 也是一个有效的格雷编码序列。

00 - 0
10 - 2
11 - 3
01 - 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 0
<strong>输出:</strong> [0]
<strong>解释:</strong> 我们定义格雷编码序列必须以 0 开头。
     给定编码总位数为 <em>n</em> 的格雷编码序列，其长度为 2<sup>n</sup>。当 <em>n</em> = 0 时，长度为 2<sup>0</sup> = 1。
     因此，当 <em>n</em> = 0 时，其格雷编码序列为 [0]。
</pre>

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![0];

        for _ in 0..n {
            let mut rev = ret.iter().rev().map(|&num| num + x).collect();
            ret.append(&mut rev);
            x *= 2;
        }

        ret
    }
}
```
