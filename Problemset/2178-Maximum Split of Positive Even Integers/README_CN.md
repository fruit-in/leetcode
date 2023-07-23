# 2178. 拆分成最多数目的正偶数之和
给你一个整数 `finalSum` 。请你将它拆分成若干个 **互不相同** 的正偶数之和，且拆分出来的正偶数数目 **最多** 。

* 比方说，给你 `finalSum = 12` ，那么这些拆分是 **符合要求** 的（互不相同的正偶数且和为 `finalSum`）：`(2 + 10)` ，`(2 + 4 + 6)` 和 `(4 + 8)` 。它们中，`(2 + 4 + 6)` 包含最多数目的整数。注意 `finalSum` 不能拆分成 `(2 + 2 + 4 + 4)` ，因为拆分出来的整数必须互不相同。

请你返回一个整数数组，表示将整数拆分成 **最多** 数目的正偶数数组。如果没有办法将 `finalSum` 进行拆分，请你返回一个 **空** 数组。你可以按 **任意** 顺序返回这些整数。

#### 示例 1:
<pre>
<strong>输入:</strong> finalSum = 12
<strong>输出:</strong> [2,4,6]
<strong>解释:</strong> 以下是一些符合要求的拆分：(2 + 10)，(2 + 4 + 6) 和 (4 + 8) 。
(2 + 4 + 6) 为最多数目的整数，数目为 3 ，所以我们返回 [2,4,6] 。
[2,6,4] ，[6,2,4] 等等也都是可行的解。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> finalSum = 7
<strong>输出:</strong> []
<strong>解释:</strong> 没有办法将 finalSum 进行拆分。
所以返回空数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> finalSum = 28
<strong>输出:</strong> [6,8,2,12]
<strong>解释:</strong> 以下是一些符合要求的拆分：(2 + 26)，(6 + 8 + 2 + 12) 和 (4 + 24) 。
(6 + 8 + 2 + 12) 有最多数目的整数，数目为 4 ，所以我们返回 [6,8,2,12] 。
[10,2,4,12] ，[6,2,4,16] 等等也都是可行的解。
</pre>

#### 提示:
* <code>1 <= finalSum <= 10<sup>10</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        if final_sum % 2 == 1 {
            return vec![];
        }

        let mut final_sum = final_sum;
        let mut ret = vec![];

        for x in (2..=final_sum).step_by(2) {
            if final_sum - x <= x {
                ret.push(final_sum);
                break;
            }

            ret.push(x);
            final_sum -= x;
        }

        ret
    }
}
```
