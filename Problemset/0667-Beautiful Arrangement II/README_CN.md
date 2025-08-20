# 667. 优美的排列 II
给你两个整数 `n` 和 `k` ，请你构造一个答案列表 `answer` ，该列表应当包含从 `1` 到 `n` 的 `n` 个不同正整数，并同时满足下述条件：
* 假设该列表是 <code>answer = [a<sub>1</sub>, a<sub>2</sub>, a<sub>3</sub>, ... , a<sub>n</sub>]</code> ，那么列表 <code>[|a<sub>1</sub> - a<sub>2</sub>|, |a<sub>2</sub> - a<sub>3</sub>|, |a<sub>3</sub> - a<sub>4</sub>|, ... , |a<sub>n-1</sub> - a<sub>n</sub>|]</code> 中应该有且仅有 `k` 个不同整数。

返回列表 `answer` 。如果存在多种答案，只需返回其中 **任意一种** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, k = 1
<strong>输出:</strong> [1,2,3]
<strong>解释:</strong> [1, 2, 3] 包含 3 个范围在 1-3 的不同整数，并且 [1, 1] 中有且仅有 1 个不同整数：1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, k = 2
<strong>输出:</strong> [1,3,2]
<strong>解释:</strong> [1, 3, 2] 包含 3 个范围在 1-3 的不同整数，并且 [2, 1] 中有且仅有 2 个不同整数：1 和 2
</pre>

#### 提示:
* <code>1 <= k < n <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut answer = vec![];

        for x in 1..=k / 2 {
            answer.push(x);
            answer.push(n + 1 - x);
        }

        if k % 2 == 1 {
            for x in k / 2 + 1..=n - k / 2 {
                answer.push(x);
            }
        } else {
            for x in (k / 2 + 1..=n - k / 2).rev() {
                answer.push(x);
            }
        }

        answer
    }
}
```
