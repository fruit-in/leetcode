# 2434. 使用机器人打印字典序最小的字符串
给你一个字符串 `s` 和一个机器人，机器人当前有一个空字符串 `t` 。执行以下操作之一，直到 `s` 和 `t` **都变成空字符串**：

* 删除字符串 `s` 的 **第一个** 字符，并将该字符给机器人。机器人把这个字符添加到 `t` 的尾部。
* 删除字符串 `t` 的 **最后一个** 字符，并将该字符给机器人。机器人将该字符写到纸上。

请你返回纸上能写出的字典序最小的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "zza"
<strong>输出:</strong> "azz"
<strong>解释:</strong> 用 p 表示写出来的字符串。
一开始，p="" ，s="zza" ，t="" 。
执行第一个操作三次，得到 p="" ，s="" ，t="zza" 。
执行第二个操作三次，得到 p="azz" ，s="" ，t="" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "bac"
<strong>输出:</strong> "abc"
<strong>解释:</strong> 用 p 表示写出来的字符串。
执行第一个操作两次，得到 p="" ，s="c" ，t="ba" 。
执行第二个操作两次，得到 p="ab" ，s="c" ，t="" 。
执行第一个操作，得到 p="ab" ，s="" ，t="c" 。
执行第二个操作，得到 p="abc" ，s="" ，t="" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "bdda"
<strong>输出:</strong> "addb"
<strong>Explanation:</strong> 用 p 表示写出来的字符串。
一开始，p="" ，s="bdda" ，t="" 。
执行第一个操作四次，得到 p="" ，s="" ，t="bdda" 。
执行第二个操作四次，得到 p="addb" ，s="" ，t="" 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut s = s.bytes().rev().collect::<Vec<_>>();
        let mut t = vec![];
        let mut p = vec![];
        let mut count = [0; 26];

        for &ch in &s {
            count[(ch - b'a') as usize] += 1;
        }

        while let Some(ch0) = s.pop() {
            count[(ch0 - b'a') as usize] -= 1;
            t.push(ch0);

            while let Some(&ch1) = t.last() {
                if count.iter().take((ch1 - b'a') as usize).all(|&x| x == 0) {
                    p.push(t.pop().unwrap());
                } else {
                    break;
                }
            }
        }

        while let Some(ch) = t.pop() {
            p.push(ch);
        }

        String::from_utf8(p).unwrap()
    }
}
```
