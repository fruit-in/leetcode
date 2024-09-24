# 2064. 分配给商店的最多商品的最小值
给你一个整数 `n` ，表示有 `n` 间零售商店。总共有 `m` 种产品，每种产品的数目用一个下标从 **0** 开始的整数数组 `quantities` 表示，其中 `quantities[i]` 表示第 `i` 种商品的数目。

你需要将 **所有商品** 分配到零售商店，并遵守这些规则：

* 一间商店 **至多** 只能有 **一种商品** ，但一间商店拥有的商品数目可以为 **任意** 件。
* 分配后，每间商店都会被分配一定数目的商品（可能为 `0` 件）。用 `x` 表示所有商店中分配商品数目的最大值，你希望 `x` 越小越好。也就是说，你想 **最小化** 分配给任意商店商品数目的 **最大值** 。

请你返回最小的可能的 `x` 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 6, quantities = [11,6]
<strong>输出:</strong> 3
<strong>解释:</strong> 一种最优方案为：
- 11 件种类为 0 的商品被分配到前 4 间商店，分配数目分别为：2，3，3，3 。
- 6 件种类为 1 的商品被分配到另外 2 间商店，分配数目分别为：3，3 。
分配给所有商店的最大商品数目为 max(2, 3, 3, 3, 3, 3) = 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 7, quantities = [15,10,10]
<strong>输出:</strong> 5
<strong>解释:</strong> 一种最优方案为：
- 15 件种类为 0 的商品被分配到前 3 间商店，分配数目为：5，5，5 。
- 10 件种类为 1 的商品被分配到接下来 2 间商店，数目为：5，5 。
- 10 件种类为 2 的商品被分配到最后 2 间商店，数目为：5，5 。
分配给所有商店的最大商品数目为 max(5, 5, 5, 5, 5, 5, 5) = 5 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 1, quantities = [100000]
<strong>输出:</strong> 100000
<strong>解释:</strong> 唯一一种最优方案为：
- 所有 100000 件商品 0 都分配到唯一的商店中。
分配给所有商店的最大商品数目为 max(100000) = 100000 。
</pre>

#### 提示:
* `m == quantities.length`
* <code>1 <= m <= n <= 10<sup>5</sup></code>
* <code>1 <= quantities[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut high = *quantities.iter().max().unwrap();

        while low < high {
            let x = (low + high) / 2;
            let mut y = 0;

            for q in &quantities {
                y += q / x;
                if q % x != 0 {
                    y += 1;
                }
            }

            if y > n {
                low = x + 1;
            } else {
                high = x;
            }
        }

        high
    }
}
```
