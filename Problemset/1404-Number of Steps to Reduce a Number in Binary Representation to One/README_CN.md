# 1404. 将二进制表示减到 1 的步骤数
给你一个以二进制形式表示的数字 `s` 。请你返回按下述规则将其减少到 1 所需要的步骤数：

* 如果当前数字为偶数，则将其除以 2 。

* 如果当前数字为奇数，则将其加上 1 。

题目保证你总是可以按上述规则将测试用例变为 1 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "1101"
<strong>输出:</strong> 6
<strong>解释:</strong> "1101" 表示十进制数 13 。
Step 1) 13 是奇数，加 1 得到 14
Step 2) 14 是偶数，除 2 得到 7
Step 3) 7  是奇数，加 1 得到 8
Step 4) 8  是偶数，除 2 得到 4
Step 5) 4  是偶数，除 2 得到 2
Step 6) 2  是偶数，除 2 得到 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "10"
<strong>输出:</strong> 1
<strong>解释:</strong> "10" 表示十进制数 2 。
Step 1) 2 是偶数，除 2 得到 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1"
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 由字符 `'0'` 或 `'1'` 组成。
* `s[0] == '1'`

## 题解 (Rust)

### 1. 题解1
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut bits = s.chars().map(|c| c == '1').collect::<VecDeque<_>>();
        let mut ret = 0;

        while bits.len() > 1 {
            if *bits.back().unwrap() {
                let mut carry = true;

                for i in (0..bits.len()).rev() {
                    if carry {
                        bits[i] = !bits[i];
                        carry = !bits[i];
                    } else {
                        break;
                    }
                }

                if carry {
                    bits.push_front(true);
                }
            } else {
                bits.pop_back();
            }

            ret += 1
        }

        ret
    }
}
```

### 2. 题解2
```Rust
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut carry = false;
        let mut ret = s.len() as i32 - 1;

        for bit in s.chars().rev().take(s.len() - 1) {
            if (bit == '1') ^ carry {
                carry = true;
                ret += 1;
            }
        }

        ret + carry as i32
    }
}
```
