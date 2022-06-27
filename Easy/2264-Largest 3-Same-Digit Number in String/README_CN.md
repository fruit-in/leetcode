# 2264. 字符串中最大的 3 位相同数字
给你一个字符串 `num` ，表示一个大整数。如果一个整数满足下述所有条件，则认为该整数是一个 **优质整数** ：
* 该整数是 `num` 的一个长度为 `3` 的 **子字符串** 。
* 该整数由唯一一个数字重复 `3` 次组成。

以字符串形式返回 **最大的优质整数** 。如果不存在满足要求的整数，则返回一个空字符串 `""` 。

注意：
* **子字符串** 是字符串中的一个连续字符序列。
* `num` 或优质整数中可能存在 **前导零** 。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "6777133339"
<strong>输出:</strong> "777"
<strong>解释:</strong> num 中存在两个优质整数："777" 和 "333" 。
"777" 是最大的那个，所以返回 "777" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "2300019"
<strong>输出:</strong> "000"
<strong>解释:</strong> "000" 是唯一一个优质整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = "42352338"
<strong>输出:</strong> ""
<strong>解释:</strong> 不存在长度为 3 且仅由一个唯一数字组成的整数。因此，不存在优质整数。
</pre>

#### 提示:
* `3 <= num.length <= 1000`
* `num` 仅由数字（`0` - `9`）组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.into_bytes();
        let goods = num.windows(3).filter(|n| n[0] == n[1] && n[1] == n[2]);
        let largest_good = goods.max().unwrap_or(&[]);

        String::from_utf8(largest_good.to_vec()).unwrap()
    }
}
```
