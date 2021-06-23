# 1432. 改变一个整数能得到的最大差值
给你一个整数 `num` 。你可以对它进行如下步骤恰好 **两次** ：
* 选择一个数字 `x (0 <= x <= 9)`.
* 选择另一个数字 `y (0 <= y <= 9)` 。数字 `y` 可以等于 `x` 。
* 将 `num` 中所有出现 `x` 的数位都用 `y` 替换。
* 得到的新的整数 **不能** 有前导 0 ，得到的新整数也 **不能** 是 0 。

令两次对 `num` 的操作得到的结果分别为 `a` 和 `b` 。

请你返回 `a` 和 `b` 的 **最大差值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 555
<strong>输出:</strong> 888
<strong>解释:</strong> 第一次选择 x = 5 且 y = 9 ，并把得到的新数字保存在 a 中。
第二次选择 x = 5 且 y = 1 ，并把得到的新数字保存在 b 中。
现在，我们有 a = 999 和 b = 111 ，最大差值为 888
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 9
<strong>输出:</strong> 8
<strong>解释:</strong> 第一次选择 x = 9 且 y = 9 ，并把得到的新数字保存在 a 中。
第二次选择 x = 9 且 y = 1 ，并把得到的新数字保存在 b 中。
现在，我们有 a = 9 和 b = 1 ，最大差值为 8
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = 123456
<strong>输出:</strong> 820000
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> num = 10000
<strong>输出:</strong> 80000
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> num = 9288
<strong>输出:</strong> 8700
</pre>

#### 提示:
* `1 <= num <= 10^8`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let num = num.to_string();
        let max = match num.chars().find(|&c| c < '9') {
            Some(c) => num.replace(c, "9"),
            None => num.clone(),
        };
        let min = if num.starts_with('1') {
            match num.chars().find(|&c| c > '1') {
                Some(c) => num.replace(c, "0"),
                None => num,
            }
        } else {
            num.replace(num.chars().next().unwrap(), "1")
        };

        max.parse::<i32>().unwrap() - min.parse::<i32>().unwrap()
    }
}
```
