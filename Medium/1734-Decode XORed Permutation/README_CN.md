# 1734. 解码异或后的排列
给你一个整数数组 `perm` ，它是前 `n` 个正整数的排列，且 `n` 是个 **奇数** 。

它被加密成另一个长度为 `n - 1` 的整数数组 `encoded` ，满足 `encoded[i] = perm[i] XOR perm[i + 1]` 。比方说，如果 `perm = [1,3,2]` ，那么 `encoded = [2,1]` 。

给你 `encoded` 数组，请你返回原始数组 `perm` 。题目保证答案存在且唯一。

#### 示例 1:
<pre>
<strong>输入:</strong> encoded = [3,1]
<strong>输出:</strong> [1,2,3]
<strong>解释:</strong> 如果 perm = [1,2,3] ，那么 encoded = [1 XOR 2,2 XOR 3] = [3,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> encoded = [6,5,4,6]
<strong>输出:</strong> [2,4,1,5,3]
</pre>

#### 提示:
* <code>3 <= n < 10<sup>5</sup></code>
* `n` 是奇数。
* `encoded.length == n - 1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let m = n / 2;
        let mut perm = vec![0; n];
        perm[m] = m as i32 + 1;

        for i in 0..m {
            perm[m] ^= ((i + 1) ^ (n - i)) as i32;
            if i % 2 == 0 {
                perm[m] ^= encoded[i] ^ encoded[n - 2 - i];
            }
        }

        for i in 1..=m {
            perm[m - i] = encoded[m - i] ^ perm[m - i + 1];
            perm[m + i] = encoded[m + i - 1] ^ perm[m + i - 1];
        }

        perm
    }
}
```
