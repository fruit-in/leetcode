# 1711. 大餐计数
**大餐** 是指 **恰好包含两道不同餐品** 的一餐，其美味程度之和等于 2 的幂。

你可以搭配 **任意** 两道餐品做一顿大餐。

给你一个整数数组 `deliciousness` ，其中 `deliciousness[i]` 是第 `i` 道餐品的美味程度，返回你可以用数组中的餐品做出的不同 **大餐** 的数量。结果需要对 <code>10<sup>9</sup> + 7</code> 取余。

注意，只要餐品下标不同，就可以认为是不同的餐品，即便它们的美味程度相同。

#### 示例 1:
<pre>
<strong>输入:</strong> deliciousness = [1,3,5,7,9]
<strong>输出:</strong> 4
<strong>解释:</strong> 大餐的美味程度组合为 (1,3) 、(1,7) 、(3,5) 和 (7,9) 。
它们各自的美味程度之和分别为 4 、8 、8 和 16 ，都是 2 的幂。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> deliciousness = [1,1,1,3,3,3,7]
<strong>输出:</strong> 15
<strong>解释:</strong> 大餐的美味程度组合为 3 种 (1,1) ，9 种 (1,3) ，和 3 种 (1,7) 。
</pre>

#### 提示:
* <code>1 <= deliciousness.length <= 10<sup>5</sup></code>
* <code>0 <= deliciousness[i] <= 2<sup>20</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0_i64;

        for i in 0..deliciousness.len() {
            count
                .entry(deliciousness[i])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for (&food0, c) in count.iter() {
            for i in 0..22 {
                let food1 = (1 << i) - food0;
                if food1 < food0 {
                    ret = (ret + c * count.get(&food1).unwrap_or(&0)) % 1_000_000_007;
                } else if food1 == food0 {
                    ret = (ret + c * (c - 1) / 2) % 1_000_000_007;
                } else {
                    break;
                }
            }
        }

        ret as i32
    }
}
```
