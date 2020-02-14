# 1323. 6 和 9 组成的最大数字
给你一个仅由数字 6 和 9 组成的正整数 ```num```。

你最多只能翻转一位数字，将 6 变成 9，或者把 9 变成 6 。

请返回你可以得到的最大数字。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 9669
<strong>输出:</strong> 9969
<strong>解释:</strong>
改变第一位数字可以得到 6669 。
改变第二位数字可以得到 9969 。
改变第三位数字可以得到 9699 。
改变第四位数字可以得到 9666 。
其中最大的数字是 9969 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 9996
<strong>输出:</strong> 9999
<strong>解释:</strong> 将最后一位从 6 变到 9，其结果 9999 是最大的数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = 9999
<strong>输出:</strong> 9999
<strong>解释:</strong> 无需改变就已经是最大的数字了。
</pre>

#### 提示:
* ```1 <= num <= 10^4```
* ```num``` 每一位上的数字都是 6 或者 9 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut i = 0;

        for i in (0..5).rev() {
            if num / 10_i32.pow(i) % 10 == 6 {
                return num + 3 * 10_i32.pow(i);
            }
        }

        num
    }
}
```
