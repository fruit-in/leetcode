# 2520. 统计能整除数字的位数
给你一个整数 `num` ，返回 `num` 中能整除 `num` 的数位的数目。

如果满足 `nums % val == 0` ，则认为整数 `val` 可以整除 `nums` 。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 7
<strong>输出:</strong> 1
<strong>解释:</strong> 7 被自己整除，因此答案是 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 121
<strong>输出:</strong> 2
<strong>解释:</strong> 121 可以被 1 整除，但无法被 2 整除。由于 1 出现两次，所以返回 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = 1248
<strong>输出:</strong> 4
<strong>解释:</strong> 1248 可以被它每一位上的数字整除，因此答案是 4 。
</pre>

#### 提示:
* <code>1 <= num <= 10<sup>9</sup></code>
* `num` 的数位中不含 `0`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut x = num;
        let mut ret = 0;

        while x > 0 {
            if num % (x % 10) == 0 {
                ret += 1;
            }
            x /= 10;
        }

        ret
    }
}
```
