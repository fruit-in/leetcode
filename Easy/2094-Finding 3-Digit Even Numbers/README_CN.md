# 2094. 找出 3 位偶数
给你一个整数数组 `digits` ，其中每个元素是一个数字（`0 - 9`）。数组中可能存在重复元素。

你需要找出 **所有** 满足下述条件且 **互不相同** 的整数：
* 该整数由 `digits` 中的三个元素按 **任意** 顺序 **依次连接** 组成。
* 该整数不含 **前导零**
* 该整数是一个 **偶数**

例如，给定的 `digits` 是 `[1, 2, 3]` ，整数 `132` 和 `312` 满足上面列出的全部条件。

将找出的所有互不相同的整数按 **递增顺序** 排列，并以数组形式返回。

#### 示例 1:
<pre>
<strong>输入:</strong> digits = [2,1,3,0]
<strong>输出:</strong> 102,120,130,132,210,230,302,310,312,320]
<strong>解释:</strong>
所有满足题目条件的整数都在输出数组中列出。
注意，答案数组中不含有 奇数 或带 前导零 的整数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> digits = [2,2,8,8,2]
<strong>输出:</strong> [222,228,282,288,822,828,882]
<strong>解释:</strong>
同样的数字（0 - 9）在构造整数时可以重复多次，重复次数最多与其在 digits 中出现的次数一样。
在这个例子中，数字 8 在构造 288、828 和 882 时都重复了两次。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> digits = [3,7,5]
<strong>输出:</strong> []
<strong>解释:</strong>
使用给定的 digits 无法构造偶数。
</pre>

#### 提示:
* `3 <= digits.length <= 100`
* `0 <= digits[i] <= 9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut ret = vec![];
        digits.sort_unstable();

        for i in 0..digits.len() {
            for j in 0..digits.len() {
                for k in 0..digits.len() {
                    let x = digits[i] * 100 + digits[j] * 10 + digits[k];

                    if i != j && j != k && i != k && x % 2 == 0 && x > *ret.last().unwrap_or(&99) {
                        ret.push(x);
                    }
                }
            }
        }

        ret
    }
}
```
