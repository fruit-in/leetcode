# 1750. 删除字符串两端相同字符后的最短长度
给你一个只包含字符 `'a'`，`'b'` 和 `'c'` 的字符串 `s` ，你可以执行下面这个操作（5 个步骤）任意次：
1. 选择字符串 `s` 一个 **非空** 的前缀，这个前缀的所有字符都相同。
2. 选择字符串 `s` 一个 **非空** 的后缀，这个后缀的所有字符都相同。
3. 前缀和后缀在字符串中任意位置都不能有交集。
4. 前缀和后缀包含的所有字符都要相同。
5. 同时删除前缀和后缀。

请你返回对字符串 `s` 执行上面操作任意次以后（可能 0 次），能得到的 **最短长度** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ca"
<strong>输出:</strong> 2
<strong>解释:</strong> 你没法删除任何一个字符，所以字符串长度仍然保持不变。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "cabaabac"
<strong>输出:</strong> 0
<strong>解释:</strong> 最优操作序列为：
- 选择前缀 "c" 和后缀 "c" 并删除它们，得到 s = "abaaba" 。
- 选择前缀 "a" 和后缀 "a" 并删除它们，得到 s = "baab" 。
- 选择前缀 "b" 和后缀 "b" 并删除它们，得到 s = "aa" 。
- 选择前缀 "a" 和后缀 "a" 并删除它们，得到 s = "" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aabccabba"
<strong>输出:</strong> 3
<strong>解释:</strong> 最优操作序列为：
- 选择前缀 "aa" 和后缀 "a" 并删除它们，得到 s = "bccabb" 。
- 选择前缀 "b" 和后缀 "bb" 并删除它们，得到 s = "cca" 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含字符 `'a'`，`'b'` 和 `'c'` 。

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {String} s
# @return {Integer}
def minimum_length(s)
  i = 0
  j = s.size - 1

  while i < j && s[i] == s[j]
    c = s[i]
    i += 1 while i < j && s[i] == c
    return 0 if i == j
    j -= 1 while i < j && s[j] == c
  end

  j - i + 1
end
```

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j && s[i] == s[j] {
            let c = s[i];
            while i < j && s[i] == c {
                i += 1;
            }
            if i == j {
                return 0;
            }
            while i < j && s[j] == c {
                j -= 1;
            }
        }

        (j - i + 1) as i32
    }
}
```
