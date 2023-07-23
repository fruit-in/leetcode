# 1689. 十-二进制数的最少数目
如果一个十进制数字不含任何前导零，且每一位上的数字不是 `0` 就是 `1` ，那么该数字就是一个 **十-二进制数** 。例如，`101` 和 `1100` 都是 **十-二进制数**，而 `112` 和 `3001` 不是。

给你一个表示十进制整数的字符串 `n` ，返回和为 `n` 的 **十-二进制数** 的最少数目。

#### 示例 1:
<pre>
<strong>输入:</strong> n = "32"
<strong>输出:</strong> 3
<strong>解释:</strong> 10 + 11 + 11 = 32
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = "82734"
<strong>输出:</strong> 8
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = "27346209830709182346"
<strong>输出:</strong> 9
</pre>

#### 提示:
* <code>1 <= n.length <= 10<sup>5</sup></code>
* `n` 仅由数字组成
* `n` 不含任何前导零并总是表示正整数

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} n
# @return {Integer}
def min_partitions(n)
  n.chars.max.to_i
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.bytes().max().unwrap() as i32 - 48
    }
}
```
