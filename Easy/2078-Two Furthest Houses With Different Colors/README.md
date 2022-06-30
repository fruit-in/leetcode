# 2078. Two Furthest Houses With Different Colors
There are `n` houses evenly lined up on the street, and each house is beautifully painted. You are given a **0-indexed** integer array `colors` of length `n`, where `colors[i]` represents the color of the <code>i<sup>th</sup></code> house.

Return *the **maximum** distance between **two** houses with **different** colors*.

The distance between the <code>i<sup>th</sup></code> and <code>j<sup>th</sup></code> houses is `abs(i - j)`, where `abs(x)` is the **absolute value** of `x`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/31/eg1.png)
<pre>
<strong>Input:</strong> colors = [1,1,1,6,1,1,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> In the above image, color 1 is blue, and color 6 is red.
The furthest two houses with different colors are house 0 and house 3.
House 0 has color 1, and house 3 has color 6. The distance between them is abs(0 - 3) = 3.
Note that houses 3 and 6 can also produce the optimal answer.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/31/eg2.png)
<pre>
<strong>Input:</strong> colors = [1,8,3,8,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> In the above image, color 1 is blue, color 8 is yellow, and color 3 is green.
The furthest two houses with different colors are house 0 and house 4.
House 0 has color 1, and house 4 has color 3. The distance between them is abs(0 - 4) = 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> colors = [0,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The furthest two houses with different colors are house 0 and house 1.
House 0 has color 0, and house 1 has color 1. The distance between them is abs(0 - 1) = 1.
</pre>

#### Constraints:
* `n == colors.length`
* `2 <= n <= 100`
* `0 <= colors[i] <= 100`
* Test data are generated such that **at least** two houses have different colors.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut ret = n - 1;

        while colors[0] == colors[ret] && colors[n - 1] == colors[n - 1 - ret] {
            ret -= 1;
        }

        ret as i32
    }
}
```
