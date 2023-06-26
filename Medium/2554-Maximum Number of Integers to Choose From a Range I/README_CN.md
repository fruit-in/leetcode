# 2554. 从一个范围内选择最多整数 I
给你一个整数数组 `banned` 和两个整数 `n` 和 `maxSum` 。你需要按照以下规则选择一些整数：

* 被选择整数的范围是 `[1, n]` 。
* 每个整数 **至多** 选择 一次 。
* 被选择整数不能在数组 `banned` 中。
* 被选择整数的和不超过 `maxSum` 。

请你返回按照上述规则 **最多** 可以选择的整数数目。

#### 示例 1:
<pre>
<strong>输入:</strong> banned = [1,6,5], n = 5, maxSum = 6
<strong>输出:</strong> 2
<strong>解释:</strong> 你可以选择整数 2 和 4 。
2 和 4 在范围 [1, 5] 内，且它们都不在 banned 中，它们的和是 6 ，没有超过 maxSum 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> banned = [1,2,3,4,5,6,7], n = 8, maxSum = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 按照上述规则无法选择任何整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> banned = [11], n = 7, maxSum = 50
<strong>输出:</strong> 7
<strong>解释:</strong> 你可以选择整数 1, 2, 3, 4, 5, 6 和 7 。
它们都在范围 [1, 7] 中，且都没出现在 banned 中，它们的和是 28 ，没有超过 maxSum 。
</pre>

#### 提示:
* <code>1 <= banned.length <= 10<sup>4</sup></code>
* <code>1 <= banned[i], n <= 10<sup>4</sup></code>
* <code>1 <= maxSum <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned.into_iter().collect::<HashSet<_>>();
        let mut sum = 0;
        let mut ret = 0;

        for x in 1..=n {
            if !banned.contains(&x) && sum + x <= max_sum {
                sum += x;
                ret += 1;
            } else if sum + x > max_sum {
                break;
            }
        }

        ret
    }
}
```
