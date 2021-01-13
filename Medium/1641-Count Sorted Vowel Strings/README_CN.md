# 1641. 统计字典序元音字符串的数目
给你一个整数 `n`，请返回长度为 `n` 、仅由元音 (`a`, `e`, `i`, `o`, `u`) 组成且按 **字典序排列** 的字符串数量。

字符串 `s` 按 **字典序排列** 需要满足：对于所有有效的 `i`，`s[i]` 在字母表中的位置总是与 `s[i+1]` 相同或在 `s[i+1]` 之前。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 5
<strong>解释:</strong> 仅由元音组成的 5 个字典序字符串为 ["a","e","i","o","u"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 15
<strong>解释:</strong> 仅由元音组成的 15 个字典序字符串为
["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"]
注意，"ea" 不是符合题意的字符串，因为 'e' 在字母表中的位置比 'a' 靠后
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 33
<strong>输出:</strong> 66045
</pre>

#### 提示:
* `1 <= n <= 50`

## 题解 (Ruby)

### 1. 动态规划
```Ruby
# @param {Integer} n
# @return {Integer}
def count_vowel_strings(n)
  dp = [1] * 5

  (1...n).each do |_|
    dp[1] += dp[0]
    dp[2] += dp[1]
    dp[3] += dp[2]
    dp[4] += dp[3]
  end

  dp.sum
end
```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = [1; 5];

        for _ in 1..n {
            dp[1] += dp[0];
            dp[2] += dp[1];
            dp[3] += dp[2];
            dp[4] += dp[3];
        }

        dp.iter().sum()
    }
}
```
