# 868. 二进制间距
给定一个正整数 ```N```，找到并返回 ```N``` 的二进制表示中两个连续的 1 之间的最长距离。

如果没有两个连续的 1，返回 ```0``` 。

#### 示例 1:
<pre>
<strong>输入:</strong> 22
<strong>输出:</strong> 2
<strong>解释:</strong> 
22 的二进制是 0b10110 。
在 22 的二进制表示中，有三个 1，组成两对连续的 1 。
第一对连续的 1 中，两个 1 之间的距离为 2 。
第二对连续的 1 中，两个 1 之间的距离为 1 。
答案取两个距离之中最大的，也就是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> 2
<strong>解释:</strong> 
5 的二进制是 0b101 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 6
<strong>输出:</strong> 1
<strong>解释:</strong> 
6 的二进制是 0b110 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> 8
<strong>输出:</strong> 0
<strong>解释:</strong> 
8 的二进制是 0b1000 。
在 8 的二进制表示中没有连续的 1，所以返回 0 。
</pre>

#### 提示:
* ```1 <= N <= 10^9```

## 题解 (Rust)

### 1. 单次遍历
```Rust
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut longest = 0;
        let mut gap = 0;
        while n & 1 == 0 {
            n >>= 1;
        }
        while n != 0 {
            n >>= 1;
            gap += 1;
            if n & 1 == 1 {
                longest = longest.max(gap);
                gap = 0;
            }
        }
        longest
    }
}
```

### 2. 存储索引
```Rust
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut longest = 0;
        let mut indexes = vec![0; 32];
        let mut i = 0;
        for j in 0..32 {
            if (n >> j) & 1 == 1 {
                indexes[i] = j;
                i += 1;
            }
        }
        i = 1;
        while indexes[i] > indexes[i - 1] {
            longest = longest.max(indexes[i] - indexes[i - 1]);
            i += 1;
        }
        longest
    }
}
```
