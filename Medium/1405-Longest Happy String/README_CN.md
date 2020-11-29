# 1405. 最长快乐字符串
如果字符串中不含有任何 `'aaa'`，`'bbb'` 或 `'ccc'` 这样的字符串作为子串，那么该字符串就是一个「快乐字符串」。

给你三个整数 `a`，`b`，`c`，请你返回 **任意一个** 满足下列全部条件的字符串 `s`：
* `s` 是一个尽可能长的快乐字符串。
* `s` 中 **最多** 有`a` 个字母 `'a'`、`b` 个字母 `'b'`、`c` 个字母 `'c'` 。
* `s` 中只含有 `'a'`、`'b'` 、`'c'` 三种字母。

如果不存在这样的字符串 `s` ，请返回一个空字符串 `""`。

#### 示例 1:
<pre>
<strong>输入:</strong> a = 1, b = 1, c = 7
<strong>输出:</strong> "ccaccbcc"
<strong>解释:</strong> "ccbccacc" 也是一种正确答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = 2, b = 2, c = 1
<strong>输出:</strong> "aabbc"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> a = 7, b = 1, c = 0
<strong>输出:</strong> "aabaa"
<strong>解释:</strong> 这是该测试用例的唯一正确答案。
</pre>

#### 提示:
* `0 <= a, b, c <= 100`
* `a + b + c > 0`

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut abc = vec![a, b, c];
        let mut prev = 0;
        let mut v = vec![(a, 'a'), (b, 'b'), (c, 'c')];
        let mut ret = vec![];

        while let Some((mut x, ch)) = v.into_iter().filter(|t| t.0 > 0).max_by_key(|t| t.0) {
            ret.push(ch);
            x -= 1;
            if x >= prev && x > 0 {
                ret.push(ch);
                x -= 1;
            }

            abc[ch as usize - 97] = x;
            prev = x;
            v = match ch {
                'a' => vec![(abc[1], 'b'), (abc[2], 'c')],
                'b' => vec![(abc[0], 'a'), (abc[2], 'c')],
                _ => vec![(abc[0], 'a'), (abc[1], 'b')],
            };
        }

        ret.iter().collect()
    }
}
```
