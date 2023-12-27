# 2048. 下一个更大的数值平衡数
如果整数  `x` 满足：对于每个数位 `d` ，这个数位 **恰好** 在 `x` 中出现 `d` 次。那么整数 `x` 就是一个 **数值平衡数** 。

给你一个整数 `n` ，请你返回 **严格大于** `n` 的 **最小数值平衡数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 22
<strong>解释:</strong>
22 是一个数值平衡数，因为：
- 数字 2 出现 2 次
这也是严格大于 1 的最小数值平衡数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1000
<strong>输出:</strong> 1333
<strong>解释:</strong>
1333 是一个数值平衡数，因为：
- 数字 1 出现 1 次。
- 数字 3 出现 3 次。
这也是严格大于 1000 的最小数值平衡数。
注意，1022 不能作为本输入的答案，因为数字 0 的出现次数超过了 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3000
<strong>输出:</strong> 3133
<strong>解释:</strong>
3133 是一个数值平衡数，因为：
- 数字 1 出现 1 次。
- 数字 3 出现 3 次。
这也是严格大于 3000 的最小数值平衡数。
</pre>

#### 提示:
* <code>0 <= n <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut x = n + 1;

        loop {
            let mut count = [0; 10];
            let mut y = x as usize;

            while y > 0 {
                count[y % 10] += 1;
                y /= 10;
            }

            if (0..10).all(|i| count[i] == i || count[i] == 0) {
                return x;
            }

            x += 1;
        }

        unreachable!()
    }
}
```
