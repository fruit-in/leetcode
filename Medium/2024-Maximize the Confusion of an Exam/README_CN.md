# 2024. 考试的最大困扰度
一位老师正在出一场由 `n` 道判断题构成的考试，每道题的答案为 true （用 `'T'` 表示）或者 false （用 `'F'` 表示）。老师想增加学生对自己做出答案的不确定性，方法是 **最大化** 有 **连续相同** 结果的题数。（也就是连续出现 true 或者连续出现 false）。

给你一个字符串 `answerKey` ，其中 `answerKey[i]` 是第 `i` 个问题的正确结果。除此以外，还给你一个整数 `k` ，表示你能进行以下操作的最多次数：

* 每次操作中，将问题的正确答案改为 `'T'` 或者 `'F'` （也就是将 `answerKey[i]` 改为 `'T'` 或者 `'F'` ）。

请你返回在不超过 `k` 次操作的情况下，**最大** 连续 `'T'` 或者 `'F'` 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> answerKey = "TTFF", k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 我们可以将两个 'F' 都变为 'T' ，得到 answerKey = "TTTT" 。
总共有四个连续的 'T' 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> answerKey = "TFFT", k = 1
<strong>输出:</strong> 3
<strong>解释:</strong> 我们可以将最前面的 'T' 换成 'F' ，得到 answerKey = "FFFT" 。
或者，我们可以将第二个 'T' 换成 'F' ，得到 answerKey = "TFFF" 。
两种情况下，都有三个连续的 'F' 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> answerKey = "TTFTTFTT", k = 1
<strong>输出:</strong> 5
<strong>解释:</strong> 我们可以将第一个 'F' 换成 'T' ，得到 answerKey = "TTTTTFTT" 。
或者我们可以将第二个 'F' 换成 'T' ，得到 answerKey = "TTFTTTTT" 。
两种情况下，都有五个连续的 'T' 。
</pre>

#### 提示:
* `n == answerKey.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* `answerKey[i]` 要么是 `'T'` ，要么是 `'F'`
* `1 <= k <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.as_bytes();
        let mut ret = 0;

        for ch in [b'T', b'F'] {
            let mut i = 0;
            let mut count = 0;

            for j in 0..answer_key.len() {
                count += (answer_key[j] == ch) as i32;
                while count > k {
                    count -= (answer_key[i] == ch) as i32;
                    i += 1;
                }
                ret = ret.max(j - i + 1);
            }
        }

        ret as i32
    }
}
```
