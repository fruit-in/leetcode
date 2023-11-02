# 216. 组合总和 III
找出所有相加之和为 `n` 的 `k` 个数的组合，且满足下列条件：

* 只使用数字1到9
* 每个数字 **最多使用一次**

返回 *所有可能的有效组合的列表* 。该列表不能包含相同的组合两次，组合可以以任何顺序返回。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 3, n = 7
<strong>输出:</strong> [[1,2,4]]
<strong>解释:</strong>
1 + 2 + 4 = 7
没有其他符合的组合了。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 3, n = 9
<strong>输出:</strong> [[1,2,6],[1,3,5],[2,3,4]]
<strong>解释:</strong>
1 + 2 + 6 = 9
1 + 3 + 5 = 9
2 + 3 + 4 = 9
没有其他符合的组合了。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> k = 4, n = 1
<strong>输出:</strong> []
<strong>解释:</strong> 不存在有效的组合。
在[1,9]范围内使用4个不同的数字，我们可以得到的最小和是1+2+3+4 = 10，因为10 > 1，没有有效的组合。
</pre>

#### 提示:
* `2 <= k <= 9`
* `1 <= n <= 60`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut x: i32 = (1 << k) - 1;
        let mut ret = vec![];

        while x < (1 << 9) {
            let comb = (1..=9)
                .filter(|&digit| (1 << (digit - 1)) & x != 0)
                .collect::<Vec<i32>>();

            if comb.iter().sum::<i32>() == n {
                ret.push(comb);
            }

            x += (x & -x) + (1 << ((x >> x.trailing_zeros()).trailing_ones() - 1)) - 1;
        }

        ret
    }
}
```
