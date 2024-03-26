# 402. 移掉 K 位数字
给你一个以字符串表示的非负整数 `num` 和一个整数 `k` ，移除这个数中的 `k` 位数字，使得剩下的数字最小。请你以字符串形式返回这个最小的数字。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "1432219", k = 3
<strong>输出:</strong> "1219"
<strong>解释:</strong> 移除掉三个数字 4, 3, 和 2 形成一个新的最小的数字 1219 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "10200", k = 1
<strong>输出:</strong> "200"
<strong>解释:</strong> 移掉首位的 1 剩下的数字为 200. 注意输出不能有任何前导零。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = "10", k = 2
<strong>输出:</strong> "0"
<strong>解释:</strong> 从原数字移除所有的数字，剩余为空就是 0 。
</pre>

#### 提示:
* <code>1 <= k <= num.length <= 10<sup>5</sup></code>
* `num` 仅由若干位数字（0 - 9）组成
* 除了 **0** 本身之外，`num` 不含任何前导零

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let keep = num.len() - k as usize;
        let mut stack = vec![];

        for (i, digit) in num.bytes().enumerate() {
            while *stack.last().unwrap_or(&b'0') > digit && num.len() - i + stack.len() > keep {
                stack.pop();
            }

            if stack.len() < keep {
                stack.push(digit);
            }
        }

        if stack.iter().all(|&digit| digit == b'0') {
            return "0".to_string();
        }

        String::from_utf8(stack)
            .unwrap()
            .trim_start_matches('0')
            .to_string()
    }
}
```
