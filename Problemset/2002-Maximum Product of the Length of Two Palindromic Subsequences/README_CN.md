# 2002. 两个回文子序列长度的最大乘积
给你一个字符串 `s` ，请你找到 `s` 中两个 **不相交回文子序列** ，使得它们长度的 **乘积最大** 。两个子序列在原字符串中如果没有任何相同下标的字符，则它们是 **不相交** 的。

请你返回两个回文子序列长度可以达到的 **最大乘积** 。

**子序列** 指的是从原字符串中删除若干个字符（可以一个也不删除）后，剩余字符不改变顺序而得到的结果。如果一个字符串从前往后读和从后往前读一模一样，那么这个字符串是一个 **回文字符串** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/24/two-palindromic-subsequences.png)
<pre>
<strong>输入:</strong> s = "leetcodecom"
<strong>输出:</strong> 9
<strong>解释:</strong> 最优方案是选择 "ete" 作为第一个子序列，"cdc" 作为第二个子序列。
它们的乘积为 3 * 3 = 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "bb"
<strong>输出:</strong> 1
<strong>解释:</strong> 最优方案为选择 "b" （第一个字符）作为第一个子序列，"b" （第二个字符）作为第二个子序列。
它们的乘积为 1 * 1 = 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "accbcaxxcxx"
<strong>输出:</strong> 25
<strong>解释:</strong> 最优方案为选择 "accca" 作为第一个子序列，"xxcxx" 作为第二个子序列。
它们的乘积为 5 * 5 = 25 。
</pre>

#### 提示:
* `2 <= s.length <= 12`
* `s` 只含有小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let s = s.as_bytes();
        let mut palindromic_subs = vec![];
        let mut ret = 0;

        for x in 1_i32..(1 << s.len()) {
            let mut sub = vec![];

            for i in 0..s.len() {
                if x & (1 << i) != 0 {
                    sub.push(s[i]);
                }
            }

            for i in 0..=sub.len() / 2 {
                if sub[i] != sub[sub.len() - 1 - i] {
                    break;
                }
                if i == sub.len() / 2 {
                    for y in &palindromic_subs {
                        if x & y == 0 {
                            ret = ret.max(x.count_ones() * y.count_ones());
                        }
                    }
                    palindromic_subs.push(x);
                }
            }
        }

        ret as i32
    }
}
```
