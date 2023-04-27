# 2390. 从字符串中移除星号
给你一个包含若干星号 `*` 的字符串 `s` 。

在一步操作中，你可以：

* 选中 `s` 中的一个星号。
* 移除星号 **左侧** 最近的那个 **非星号** 字符，并移除该星号自身。

返回移除 **所有** 星号之后的字符串。

#### 注意：
* 生成的输入保证总是可以执行题面中描述的操作。
* 可以证明结果字符串是唯一的。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leet**cod*e"
<strong>输出:</strong> "lecoe"
<strong>解释:</strong> 从左到右执行移除操作：
- 距离第 1 个星号最近的字符是 "leet**cod*e" 中的 't' ，s 变为 "lee*cod*e" 。
- 距离第 2 个星号最近的字符是 "lee*cod*e" 中的 'e' ，s 变为 "lecod*e" 。
- 距离第 3 个星号最近的字符是 "lecod*e" 中的 'd' ，s 变为 "lecoe" 。
不存在其他星号，返回 "lecoe" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "erase*****"
<strong>输出:</strong> ""
<strong>解释:</strong> 整个字符串都会被移除，所以返回空字符串。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 由小写英文字母和星号 `*` 组成
* `s` 可以执行上述操作

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ret = vec![];

        for b in s.bytes() {
            if b == b'*' {
                ret.pop();
            } else {
                ret.push(b);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
