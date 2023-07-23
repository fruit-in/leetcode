# 1720. 解码异或后的数组
**未知** 整数数组 `arr` 由 `n` 个非负整数组成。

经编码后变为长度为 `n - 1` 的另一个整数数组 `encoded` ，其中 `encoded[i] = arr[i] XOR arr[i + 1]` 。例如，`arr = [1,0,2,1]` 经编码后得到 `encoded = [1,2,3]` 。

给你编码后的数组 `encoded` 和原数组 `arr` 的第一个元素 `first`（`arr[0]`）。

请解码返回原数组 `arr` 。可以证明答案存在并且是唯一的。

#### 示例 1:
<pre>
<strong>输入:</strong> encoded = [1,2,3], first = 1
<strong>输出:</strong> [1,0,2,1]
<strong>解释:</strong> 若 arr = [1,0,2,1] ，那么 first = 1 且 encoded = [1 XOR 0, 0 XOR 2, 2 XOR 1] = [1,2,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> encoded = [6,2,7,3], first = 4
<strong>输出:</strong> [4,2,0,7,4]
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>4</sup></code>
* `encoded.length == n - 1`
* <code>0 <= encoded[i] <= 10<sup>5</sup></code>
* <code>0 <= first <= 10<sup>5</sup></code>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} encoded
# @param {Integer} first
# @return {Integer[]}
def decode(encoded, first)
  ret = [first]

  encoded.each do |n|
    first ^= n
    ret.push(first)
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        let mut ret = vec![first];

        for n in encoded {
            first ^= n;
            ret.push(first);
        }

        ret
    }
}
```
