# 791. 自定义字符串排序
字符串`S`和 `T` 只包含小写字符。在`S`中，所有字符只会出现一次。

`S` 已经根据某种规则进行了排序。我们要根据`S`中的字符顺序对`T`进行排序。更具体地说，如果`S`中`x`在`y`之前出现，那么返回的字符串中`x`也应出现在`y`之前。

返回任意一种符合条件的字符串`T`。

#### 示例:
<pre>
<b>输入:</b>
S = "cba"
T = "abcd"
<b>输出:</b> "cbad"
<b>解释:</b>
S中出现了字符 "a", "b", "c", 所以 "a", "b", "c" 的顺序应该是 "c", "b", "a".
由于 "d" 没有在S中出现, 它可以放在T的任意位置. "dcba", "cdba", "cbda" 都是合法的输出。
</pre>

#### 注意:
* `S`的最大长度为`26`，其中没有重复的字符。
* `T`的最大长度为`200`。
* `S`和`T`只包含小写字符。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let mut indices = [26; 26];
        let mut t = t.into_bytes();

        for (i, ch) in s.bytes().enumerate() {
            indices[(ch - b'a') as usize] = i;
        }
        t.sort_unstable_by_key(|k| indices[(k - b'a') as usize]);

        String::from_utf8(t).unwrap()
    }
}
```
