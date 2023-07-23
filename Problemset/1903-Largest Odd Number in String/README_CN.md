# 1903. 字符串中的最大奇数
给你一个字符串 `num` ，表示一个大整数。请你在字符串 `num` 的所有 **非空子字符串** 中找出 **值最大的奇数** ，并以字符串形式返回。如果不存在奇数，则返回一个空字符串 `""` 。

**子字符串** 是字符串中的一个连续的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "52"
<strong>输出:</strong> "5"
<strong>解释:</strong> 非空子字符串仅有 "5"、"2" 和 "52" 。"5" 是其中唯一的奇数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "4206"
<strong>输出:</strong> ""
<strong>解释:</strong> 在 "4206" 中不存在奇数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = "35427"
<strong>输出:</strong> "35427"
<strong>解释:</strong> "35427" 本身就是一个奇数。
</pre>

#### 提示:
* <code>1 <= num.length <= 10<sup>5</sup></code>
* `num` 仅由数字组成且不含前导零

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut num = num.into_bytes();

        while *num.last().unwrap_or(&1) % 2 == 0 {
            num.pop();
        }

        String::from_utf8(num).unwrap()
    }
}
```
