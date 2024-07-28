# 2310. 个位数字为 K 的整数之和
给你两个整数 `num` 和 `k` ，考虑具有以下属性的正整数多重集：

* 每个整数个位数字都是 `k` 。
* 所有整数之和是 `num` 。

返回该多重集的最小大小，如果不存在这样的多重集，返回 `-1` 。

注意：

* 多重集与集合类似，但多重集可以包含多个同一整数，空多重集的和为 `0` 。
* **个位数字** 是数字最右边的数位。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 58, k = 9
<strong>输出:</strong> 2
<strong>解释:</strong>
多重集 [9,49] 满足题目条件，和为 58 且每个整数的个位数字是 9 。
另一个满足条件的多重集是 [19,39] 。
可以证明 2 是满足题目条件的多重集的最小长度。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 37, k = 2
<strong>输出:</strong> -1
<strong>解释:</strong> 个位数字为 2 的整数无法相加得到 37 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = 0, k = 7
<strong>输出:</strong> 0
<strong>解释:</strong> 空多重集的和为 0 。
</pre>

#### 提示:
* `0 <= num <= 3000`
* `0 <= k <= 9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        let mut sum = k;

        for i in 1..=10 {
            if sum <= num && sum % 10 == num % 10 {
                return i;
            }
            sum += k;
        }

        -1
    }
}
```
