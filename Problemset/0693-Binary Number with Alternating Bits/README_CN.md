# 693. 交替位二进制数
给定一个正整数，检查他是否为交替位二进制数：换句话说，就是他的二进制数相邻的两个位数永不相等。

#### 示例 1:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> True
<strong>解释:</strong>
5的二进制数是: 101
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 7
<strong>Output:</strong> False
<strong>解释:</strong>
7的二进制数是: 111
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 11
<strong>输出:</strong> False
<strong>解释:</strong>
11的二进制数是: 1011
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> 10
<strong>输出:</strong> True
<strong>解释:</strong>
10的二进制数是: 1010
</pre>

## 题解 (Rust)

### 1. 移位并比较最后两位
```Rust
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut pre = n & 1;
        while n != 0 {
            n >>= 1;
            if pre == n & 1 {
                return false;
            }
            pre = n & 1;
        }

        true
    }
}
```

### 2. 计算有效交替位二进制数
```Rust
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut i = 1;
        while i > 0 && i < n {
            match i % 2 {
                1 => i = 2 * i,
                _ => i = 2 * i + 1,
            };
        }

        i == n
    }
}
```
