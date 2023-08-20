# 2515. 到目标字符串的最短距离
给你一个下标从 **0** 开始的 **环形** 字符串数组 `words` 和一个字符串 `target` 。**环形数组** 意味着数组首尾相连。

* 形式上， `words[i]` 的下一个元素是 `words[(i + 1) % n]` ，而 `words[i]` 的前一个元素是 `words[(i - 1 + n) % n]` ，其中 `n` 是 `words` 的长度。

从 `startIndex` 开始，你一次可以用 `1` 步移动到下一个或者前一个单词。

返回到达目标字符串 `target` 所需的最短距离。如果 `words` 中不存在字符串 `target` ，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["hello","i","am","leetcode","hello"], target = "hello", startIndex = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 从下标 1 开始，可以经由以下步骤到达 "hello" ：
- 向右移动 3 个单位，到达下标 4 。
- 向左移动 2 个单位，到达下标 4 。
- 向右移动 4 个单位，到达下标 0 。
- 向左移动 1 个单位，到达下标 0 。
到达 "hello" 的最短距离是 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["a","b","leetcode"], target = "leetcode", startIndex = 0
<strong>输出:</strong> 1
<strong>解释:</strong> 从下标 0 开始，可以经由以下步骤到达 "leetcode" ：
- 向右移动 2 个单位，到达下标 3 。
- 向左移动 1 个单位，到达下标 3 。
到达 "leetcode" 的最短距离是 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["i","eat","leetcode"], target = "ate", start输入 = 0
<strong>输出:</strong> -1
<strong>解释:</strong> 因为 words 中不存在字符串 "ate" ，所以返回 -1 。
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` 和 `target` 仅由小写英文字母组成
* `0 <= startIndex < words.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_index = start_index as usize;
        let n = words.len();

        for i in 0..n {
            if words[(start_index + i) % n] == target || words[(start_index + n - i) % n] == target
            {
                return i as i32;
            }
        }

        -1
    }
}
```
