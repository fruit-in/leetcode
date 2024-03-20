# 1840. Maximum Building Height
You want to build `n` new buildings in a city. The new buildings will be built in a line and are labeled from `1` to `n`.

However, there are city restrictions on the heights of the new buildings:

* The height of each building must be a non-negative integer.
* The height of the first building **must** be `0`.
* The height difference between any two adjacent buildings **cannot exceed** `1`.

Additionally, there are city restrictions on the maximum height of specific buildings. These restrictions are given as a 2D integer array `restrictions` where <code>restrictions[i] = [id<sub>i</sub>, maxHeight<sub>i</sub>]</code> indicates that building <code>id<sub>i</sub></code> must have a height **less than or equal to** <code>maxHeight<sub>i</sub></code>.

It is guaranteed that each building will appear **at most once** in `restrictions`, and building `1` will **not** be in `restrictions`.

Return *the **maximum possible height** of the **tallest** building*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex1-1.png)
<pre>
<strong>Input:</strong> n = 5, restrictions = [[2,1],[4,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The green area in the image indicates the maximum allowed height for each building.
We can build the buildings with heights [0,1,2,1,2], and the tallest building has a height of 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex2.png)
<pre>
<strong>Input:</strong> n = 6, restrictions = []
<strong>Output:</strong> 5
<strong>Explanation:</strong> The green area in the image indicates the maximum allowed height for each building.
We can build the buildings with heights [0,1,2,3,4,5], and the tallest building has a height of 5.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/04/08/ic236-q4-ex3.png)
<pre>
<strong>Input:</strong> n = 10, restrictions = [[5,3],[2,5],[7,4],[10,3]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The green area in the image indicates the maximum allowed height for each building.
We can build the buildings with heights [0,1,2,3,3,4,4,5,4,3], and the tallest building has a height of 5.
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>9</sup></code>
* <code>0 <= restrictions.length <= min(n - 1, 10<sup>5</sup>)</code>
* <code>2 <= id<sub>i</sub> <= n</code>
* <code>id<sub>i</sub></code> is **unique**.
* <code>0 <= maxHeight<sub>i</sub> <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
        restrictions.push(vec![1, 0]);
        restrictions.sort_unstable();
        if restrictions.last().unwrap()[0] != n {
            restrictions.push(vec![n, n - 1]);
        }

        for i in 1..restrictions.len() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i - 1][1] + restrictions[i][0] - restrictions[i - 1][0]);
        }
        for i in (0..restrictions.len() - 1).rev() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i + 1][1] + restrictions[i + 1][0] - restrictions[i][0]);
        }

        (0..restrictions.len() - 1)
            .map(|i| {
                (restrictions[i + 1][1] + restrictions[i][1] - restrictions[i][0]
                    + restrictions[i + 1][0])
                    / 2
            })
            .max()
            .unwrap()
    }
}
```
