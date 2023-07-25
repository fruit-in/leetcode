# 2433. 找出前缀异或的原始数组
给你一个长度为 `n` 的 **整数** 数组 `pref` 。找出并返回满足下述条件且长度为 `n` 的数组 `arr` ：

* `pref[i] = arr[0] ^ arr[1] ^ ... ^ arr[i]`.

注意 `^` 表示 **按位异或**（bitwise-xor）运算。

可以证明答案是 **唯一** 的。

#### 示例 1:
<pre>
<strong>输入:</strong> pref = [5,2,0,3,1]
<strong>输出:</strong> [5,7,2,3,2]
<strong>解释:</strong> 从数组 [5,7,2,3,2] 可以得到如下结果：
- pref[0] = 5
- pref[1] = 5 ^ 7 = 2
- pref[2] = 5 ^ 7 ^ 2 = 0
- pref[3] = 5 ^ 7 ^ 2 ^ 3 = 3
- pref[4] = 5 ^ 7 ^ 2 ^ 3 ^ 2 = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pref = [13]
<strong>输出:</strong> [13]
<strong>解释:</strong> pref[0] = arr[0] = 13
</pre>

#### 提示:
* <code>1 <= pref.length <= 10<sup>5</sup></code>
* <code>0 <= pref[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = pref;

        for i in (1..arr.len()).rev() {
            arr[i] ^= arr[i - 1];
        }

        arr
    }
}
```
