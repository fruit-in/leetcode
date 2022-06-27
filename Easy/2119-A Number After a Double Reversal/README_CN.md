# 2119. 反转两次的数字
**反转** 一个整数意味着倒置它的所有位。
* 例如，反转 `2021` 得到 `1202` 。反转 `12300` 得到 `321` ，**不保留前导零** 。

给你一个整数 `num` ，**反转** `num` 得到 `reversed1` ，**接着反转** `reversed1` 得到 `reversed2` 。如果 `reversed2` 等于 `num` ，返回 `true` ；否则，返回 `false` 。

#### 解释 1:
<pre>
<strong>输入:</strong> num = 526
<strong>输出:</strong> true
<strong>解释:</strong> 反转 num 得到 625 ，接着反转 625 得到 526 ，等于 num 。
</pre>

#### 解释 2:
<pre>
<strong>输入:</strong> num = 1800
<strong>输出:</strong> false
<strong>解释:</strong> 反转 num 得到 81 ，接着反转 81 得到 18 ，不等于 num 。
</pre>

#### 解释 3:
<pre>
<strong>输入:</strong> num = 0
<strong>输出:</strong> true
<strong>解释:</strong> 反转 num 得到 0 ，接着反转 0 得到 0 ，等于 num 。
</pre>

#### 提示:
* <code>0 <= num <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}
```
