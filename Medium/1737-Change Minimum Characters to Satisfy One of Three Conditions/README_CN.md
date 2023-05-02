# 1737. 满足三条件之一需改变的最少字符数
给你两个字符串 `a` 和 `b` ，二者均由小写字母组成。一步操作中，你可以将 `a` 或 `b` 中的 **任一字符** 改变为 **任一小写字母** 。

操作的最终目标是满足下列三个条件 **之一** ：

* `a` 中的 **每个字母** 在字母表中 **严格小于** `b` 中的 **每个字母** 。
* `b` 中的 **每个字母** 在字母表中 **严格小于** `a` 中的 **每个字母** 。
* `a` 和 `b` **都** 由 **同一个** 字母组成。

返回达成目标所需的 **最少** 操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> a = "aba", b = "caa"
<strong>输出:</strong> 2
<strong>解释:</strong> 满足每个条件的最佳方案分别是：
1) 将 b 变为 "ccc"，2 次操作，满足 a 中的每个字母都小于 b 中的每个字母；
2) 将 a 变为 "bbb" 并将 b 变为 "aaa"，3 次操作，满足 b 中的每个字母都小于 a 中的每个字母；
3) 将 a 变为 "aaa" 并将 b 变为 "aaa"，2 次操作，满足 a 和 b 由同一个字母组成。
最佳的方案只需要 2 次操作（满足条件 1 或者条件 3）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = "dabadd", b = "cda"
<strong>输出:</strong> 3
<strong>解释:</strong> 满足条件 1 的最佳方案是将 b 变为 "eee" 。
</pre>

#### 提示:
* <code>1 <= a.length, b.length <= 10<sup>5</sup></code>
* `a` 和 `b` 只由小写字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut ret = a
            .chars()
            .chain(b.chars())
            .filter(|&chab| chab != 'a')
            .count();

        for ch in 'b'..='z' {
            ret = ret.min(
                a.chars().filter(|&cha| cha >= ch).count()
                    + b.chars().filter(|&chb| chb < ch).count(),
            );
            ret = ret.min(
                a.chars().filter(|&cha| cha < ch).count()
                    + b.chars().filter(|&chb| chb >= ch).count(),
            );
            ret = ret.min(
                a.chars()
                    .chain(b.chars())
                    .filter(|&chab| chab != ch)
                    .count(),
            );
        }

        ret as i32
    }
}
```
