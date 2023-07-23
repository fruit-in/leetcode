# 1974. 使用特殊打字机键入单词的最少时间
有一个特殊打字机，它由一个 **圆盘** 和一个 **指针** 组成， 圆盘上标有小写英文字母 `'a'` 到 `'z'`。只有 当指针指向某个字母时，它才能被键入。指针 **初始时** 指向字符 `'a'` 。

![](https://assets.leetcode.com/uploads/2021/07/31/chart.jpg)

每一秒钟，你可以执行以下操作之一：
* 将指针 **顺时针** 或者 **逆时针** 移动一个字符。
* 键入指针 **当前** 指向的字符。

给你一个字符串 `word` ，请你返回键入 `word` 所表示单词的 **最少** 秒数 。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "abc"
<strong>输出:</strong> 5
<strong>解释:</strong>
单词按如下操作键入：
- 花 1 秒键入字符 'a' in 1 ，因为指针初始指向 'a' ，故不需移动指针。
- 花 1 秒将指针顺时针移到 'b' 。
- 花 1 秒键入字符 'b' 。
- 花 1 秒将指针顺时针移到 'c' 。
- 花 1 秒键入字符 'c' 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "bza"
<strong>输出:</strong> 7
<strong>解释:</strong>
单词按如下操作键入：
- 花 1 秒将指针顺时针移到 'b' 。
- 花 1 秒键入字符 'b' 。
- 花 2 秒将指针逆时针移到 'z' 。
- 花 1 秒键入字符 'z' 。
- 花 1 秒将指针顺时针移到 'a' 。
- 花 1 秒键入字符 'a' 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "zjpc"
<strong>输出:</strong> 34
<strong>解释:</strong>
单词按如下操作键入：
- 花 1 秒将指针逆时针移到 'z' 。
- 花 1 秒键入字符 'z' 。
- 花 10 秒将指针顺时针移到 'j' 。
- 花 1 秒键入字符 'j' 。
- 花 6 秒将指针顺时针移到 'p' 。
- 花 1 秒键入字符 'p' 。
- 花 13 秒将指针逆时针移到 'c' 。
- 花 1 秒键入字符 'c' 。
</pre>

#### 提示:
* `1 <= word.length <= 100`
* `word` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut curr = b'a' as i32;
        let mut ret = 0;

        for c in word.bytes() {
            let (s, l) = (curr.min(c as i32), curr.max(c as i32));
            ret += 1 + (l - s).min(s + 26 - l);
            curr = c as i32;
        }

        ret
    }
}
```
