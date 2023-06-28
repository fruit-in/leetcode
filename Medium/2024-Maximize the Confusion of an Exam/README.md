# 2024. Maximize the Confusion of an Exam
A teacher is writing a test with `n` true/false questions, with `'T'` denoting true and `'F'` denoting false. He wants to confuse the students by **maximizing** the number of **consecutive** questions with the **same** answer (multiple trues or multiple falses in a row).

You are given a string `answerKey`, where `answerKey[i]` is the original answer to the <code>i<sup>th</sup></code> question. In addition, you are given an integer `k`, the maximum number of times you may perform the following operation:

* Change the answer key for any question to `'T'` or `'F'` (i.e., set `answerKey[i]` to `'T'` or `'F'`).

Return *the **maximum** number of consecutive* `'T'`s or `'F'`s *in the answer key after performing the operation at most* `k` *times*.

#### Example 1:
<pre>
<strong>Input:</strong> answerKey = "TTFF", k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can replace both the 'F's with 'T's to make answerKey = "TTTT".
There are four consecutive 'T's.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> answerKey = "TFFT", k = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can replace the first 'T' with an 'F' to make answerKey = "FFFT".
Alternatively, we can replace the second 'T' with an 'F' to make answerKey = "TFFF".
In both cases, there are three consecutive 'F's.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> answerKey = "TTFTTFTT", k = 1
<strong>Output:</strong> 5
<strong>Explanation:</strong> We can replace the first 'F' to make answerKey = "TTTTTFTT"
Alternatively, we can replace the second 'F' to make answerKey = "TTFTTTTT".
In both cases, there are five consecutive 'T's.
</pre>

#### Constraints:
* `n == answerKey.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* `answerKey[i]` is either `'T'` or `'F'`
* `1 <= k <= n`

## Solutions (Rust)

### 1. Solution
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
