# 1545. 找出第 N 个二进制字符串中的第 K 位
给你两个正整数 `n` 和 `k`，二进制字符串  <code>S<sub>n</sub></code> 的形成规则如下：
* <code>S<sub>1</sub> = "0"</code>
* 当 `i > 1` 时，<code>S<sub>i</sub> = S<sub>i-1</sub> + "1" + reverse(invert(S<sub>i-1</sub>))</code>

其中 `+` 表示串联操作，`reverse(x)` 返回反转 `x` 后得到的字符串，而 `invert(x)` 则会翻转 `x` 中的每一位（0 变为 1，而 1 变为 0）

例如，符合上述描述的序列的前 4 个字符串依次是：
* <code>S<sub>1</sub> = "0"</code>
* <code>S<sub>2</sub> = "0<b>1</b>1"</code>
* <code>S<sub>3</sub> = "011<b>1</b>001"</code>
* <code>S<sub>4</sub> = "0111001<b>1</b>0110001"</code>

请你返回  <code>S<sub>n</sub></code> 的 **第 `k` 位字符** ，题目数据保证 `k` 一定在 <code>S<sub>n</sub></code> 长度范围以内。

#### 示例 1:
<pre>
<b>示例:</b> n = 3, k = 1
<b>输出:</b> "0"
<b>解释:</b> S<sub>3</sub> 为 "<b>0</b>111001"，其第 1 位为 "0" 。
</pre>


#### 示例 2:
<pre>
<b>示例:</b> n = 4, k = 11
<b>输出:</b> "1"
<b>解释:</b> S<sub>4</sub> 为 "0111001101<b>1</b>0001"，其第 11 位为 "1" 。
</pre>


#### 示例 3:
<pre>
<b>示例:</b> n = 1, k = 1
<b>输出:</b> "0"
</pre>

#### 示例 4:
<pre>
<b>示例:</b> n = 2, k = 3
<b>输出:</b> "1"
</pre>

#### 提示:
* `1 <= n <= 20`
* <code>1 <= k <= 2<sup>n</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut bits = vec![false];

        loop {
            if let Some(&b) = bits.get(k as usize - 1) {
                return char::from(b as u8 + b'0');
            }

            let mut x = bits.clone().iter().map(|&b| !b).rev().collect::<Vec<_>>();
            bits.push(true);
            bits.append(&mut x);
        }
    }
}
```
