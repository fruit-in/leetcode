# 2559. 统计范围内的元音字符串数
给你一个下标从 **0** 开始的字符串数组 `words` 以及一个二维整数数组 `queries` 。

每个查询 <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> 会要求我们统计在 `words` 中下标在 <code>l<sub>i</sub></code> 到 <code>r<sub>i</sub></code> 范围内（**包含** 这两个值）并且以元音开头和结尾的字符串的数目。

返回一个整数数组，其中数组的第 `i` 个元素对应第 `i` 个查询的答案。

**注意：**元音字母是 `'a'`、`'e'`、`'i'`、`'o'` 和 `'u'` 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
<strong>输出:</strong> [2,3,0]
<strong>解释:</strong> 以元音开头和结尾的字符串是 "aba"、"ece"、"aa" 和 "e" 。
查询 [0,2] 结果为 2（字符串 "aba" 和 "ece"）。
查询 [1,4] 结果为 3（字符串 "ece"、"aa"、"e"）。
查询 [1,1] 结果为 0 。
返回结果 [2,3,0] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["a","e","i"], queries = [[0,2],[0,1],[2,2]]
<strong>输出:</strong> [3,2,1]
<strong>解释:</strong> 每个字符串都满足这一条件，所以返回 [3,2,1] 。
</pre>

#### 提示:
* <code>1 <= words.length <= 10<sup>5</sup></code>
* `1 <= words[i].length <= 40`
* `words[i]` 仅由小写英文字母组成
* <code>sum(words[i].length) <= 3 * 10<sup>5</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* <code>0 <= l<sub>i</sub> <= r<sub>i</sub> < words.length</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len() + 1];
        let mut ret = vec![0; queries.len()];

        for i in 0..words.len() {
            prefix_sum[i + 1] = prefix_sum[i];

            if words[i].starts_with(|c| "aeiou".contains(c))
                && words[i].ends_with(|c| "aeiou".contains(c))
            {
                prefix_sum[i + 1] += 1;
            }
        }

        for i in 0..queries.len() {
            ret[i] = prefix_sum[queries[i][1] as usize + 1] - prefix_sum[queries[i][0] as usize];
        }

        ret
    }
}
```
