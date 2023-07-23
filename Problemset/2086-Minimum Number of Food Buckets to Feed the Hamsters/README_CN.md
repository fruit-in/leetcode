# 2086. 从房屋收集雨水需要的最少水桶数
给你一个下标从 **0** 开始的字符串 `street` 。`street` 中每个字符要么是表示房屋的 `'H'` ，要么是表示空位的 `'.'` 。

你可以在 **空位** 放置水桶，从相邻的房屋收集雨水。位置在 `i - 1` **或者** `i + 1` 的水桶可以收集位置为 `i` 处房屋的雨水。一个水桶如果相邻两个位置都有房屋，那么它可以收集 **两个** 房屋的雨水。

在确保 **每个** 房屋旁边都 **至少** 有一个水桶的前提下，请你返回需要的 **最少** 水桶数。如果无解请返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> street = "H..H"
<strong>输出:</strong> 2
<strong>解释:</strong> 我们可以在下标为 1 和 2 处放水桶。
"H..H" -> "HBBH"（'B' 表示放置水桶）。
下标为 0 处的房屋右边有水桶，下标为 3 处的房屋左边有水桶。
所以每个房屋旁边都至少有一个水桶收集雨水。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> street = ".H.H."
<strong>输出:</strong> 1
<strong>解释:</strong>
我们可以在下标为 2 处放置一个水桶。
".H.H." -> ".HBH."（'B' 表示放置水桶）。
下标为 1 处的房屋右边有水桶，下标为 3 处的房屋左边有水桶。
所以每个房屋旁边都至少有一个水桶收集雨水。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> street = ".HHH."
<strong>输出:</strong> -1
<strong>解释:</strong>
没有空位可以放置水桶收集下标为 2 处的雨水。
所以没有办法收集所有房屋的雨水。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> street = "H."
<strong>输出:</strong> -1
<strong>解释:</strong>
没有空位放置水桶。
所以没有办法收集所有房屋的雨水。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> street = "."
<strong>输出:</strong> 0
<strong>解释:</strong>
没有房屋需要收集雨水。
所以需要 0 个水桶。
</pre>

#### 提示:
* <code>1 <= street.length <= 10<sup>5</sup></code>
* `street[i]` 要么是 `'H'` ，要么是 `'.'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let hamsters = hamsters
            .replace("H.H", "B")
            .replace(".H", "B")
            .replace("H.", "B");

        if hamsters.contains('H') {
            -1
        } else {
            hamsters.chars().filter(|&c| c == 'B').count() as i32
        }
    }
}
```
