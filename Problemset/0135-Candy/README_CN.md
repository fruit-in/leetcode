# 135. 分发糖果
`n` 个孩子站成一排。给你一个整数数组 `ratings` 表示每个孩子的评分。

你需要按照以下要求，给这些孩子分发糖果：
* 每个孩子至少分配到 `1` 个糖果。
* 相邻两个孩子评分更高的孩子会获得更多的糖果。

请你给每个孩子分发糖果，计算并返回需要准备的 **最少糖果数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> ratings = [1,0,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 你可以分别给第一个、第二个、第三个孩子分发 2、1、2 颗糖果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ratings = [1,2,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以分别给第一个、第二个、第三个孩子分发 1、2、1 颗糖果。
     第三个孩子只得到 1 颗糖果，这满足题面中的两个条件。
</pre>

#### 提示:
* `n == ratings.length`
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>0 <= ratings[i] <= 2 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.into_iter().sum()
    }
}
```
