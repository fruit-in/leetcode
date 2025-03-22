# 2561. 重排水果
你有两个果篮，每个果篮中有 `n` 个水果。给你两个下标从 **0** 开始的整数数组 `basket1` 和 `basket2` ，用以表示两个果篮中每个水果的交换成本。你想要让两个果篮相等。为此，可以根据需要多次执行下述操作：
* 选中两个下标 `i` 和 `j` ，并交换 `basket1` 中的第 `i` 个水果和 `basket2` 中的第 `j` 个水果。
* 交换的成本是 <code>min(basket1<sub>i</sub>,basket2<sub>j</sub>)</code> 。

根据果篮中水果的成本进行排序，如果排序后结果完全相同，则认为两个果篮相等。

返回使两个果篮相等的最小交换成本，如果无法使两个果篮相等，则返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> basket1 = [4,2,2,2], basket2 = [1,4,1,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 交换 basket1 中下标为 1 的水果和 basket2 中下标为 0 的水果，交换的成本为 1 。此时，basket1 = [4,1,2,2] 且 basket2 = [2,4,1,2] 。重排两个数组，发现二者相等。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> basket1 = [2,3,4,1], basket2 = [3,2,5,1]
<strong>输出:</strong> -1
<strong>解释:</strong> 可以证明无法使两个果篮相等。
</pre>

#### 提示:
* `basket1.length == basket2.length`
* <code>1 <= basket1.length <= 10<sup>5</sup></code>
* <code>1 <= basket1[i],basket2[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut min_fruit = i64::MAX;
        let mut swap = vec![];
        let mut ret = 0;

        for i in 0..basket1.len() {
            *count.entry(basket1[i]).or_insert(0_i32) += 1;
            *count.entry(basket2[i]).or_insert(0) -= 1;
            min_fruit = min_fruit.min(basket1[i].min(basket2[i]) as i64);
        }

        for (k, v) in count.into_iter() {
            if v % 2 != 0 {
                return -1;
            }

            swap.append(&mut vec![k as i64; v.abs() as usize / 2]);
        }

        swap.sort_unstable();

        swap.iter()
            .take(swap.len() / 2)
            .map(|&cost| cost.min(min_fruit * 2))
            .sum()
    }
}
```
