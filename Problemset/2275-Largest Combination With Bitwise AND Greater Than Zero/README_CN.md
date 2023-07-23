# 2275. 按位与结果大于零的最长组合
对数组 `nums` 执行 **按位与** 相当于对数组 `nums` 中的所有整数执行 **按位与** 。

* 例如，对 `nums = [1, 5, 3]` 来说，按位与等于 `1 & 5 & 3 = 1` 。
* 同样，对 `nums = [7]` 而言，按位与等于 `7` 。

给你一个正整数数组 `candidates` 。计算 `candidates` 中的数字每种组合下 **按位与** 的结果。 `candidates` 中的每个数字在每种组合中只能使用 **一次** 。

返回按位与结果大于 `0` 的 **最长** 组合的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> candidates = [16,17,71,62,12,24,14]
<strong>输出:</strong> 4
<strong>解释:</strong> 组合 [16,17,62,24] 的按位与结果是 16 & 17 & 62 & 24 = 16 > 0 。
组合长度是 4 。
可以证明不存在按位与结果大于 0 且长度大于 4 的组合。
注意，符合长度最大的组合可能不止一种。
例如，组合 [62,12,24,14] 的按位与结果是 62 & 12 & 24 & 14 = 8 > 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> candidates = [8,8]
<strong>输出:</strong> 2
<strong>解释:</strong> 最长组合是 [8,8] ，按位与结果 8 & 8 = 8 > 0 。
组合长度是 2 ，所以返回 2 。
</pre>

#### 提示:
* <code>1 <= candidates.length <= 10<sup>5</sup></code>
* <code>1 <= candidates[i] <= 10<sup>7</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut count = [0; 24];

        for &x in &candidates {
            for i in 0..24 {
                if x & (1 << i) != 0 {
                    count[i] += 1;
                }
            }
        }

        *count.iter().max().unwrap()
    }
}
```
