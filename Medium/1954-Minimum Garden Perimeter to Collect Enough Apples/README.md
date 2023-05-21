# 1954. Minimum Garden Perimeter to Collect Enough Apples
In a garden represented as an infinite 2D grid, there is an apple tree planted at **every** integer coordinate. The apple tree planted at an integer coordinate `(i, j)` has `|i| + |j|` apples growing on it.

You will buy an axis-aligned **square plot** of land that is centered at `(0, 0)`.

Given an integer `neededApples`, return *the **minimum perimeter** of a plot such that **at least*** `neededApples` *apples are **inside or on** the perimeter of that plot*.

The value of `|x|` is defined as:

* `x` if `x >= 0`
* `-x` if `x < 0`

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/08/30/1527_example_1_2.png)
<pre>
<strong>Input:</strong> neededApples = 1
<strong>Output:</strong> 8
<strong>Explanation:</strong> A square plot of side length 1 does not contain any apples.
However, a square plot of side length 2 has 12 apples inside (as depicted in the image above).
The perimeter is 2 * 4 = 8.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> neededApples = 13
<strong>Output:</strong> 16
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> neededApples = 1000000000
<strong>Output:</strong> 5040
</pre>

#### Constraints:
* <code>1 <= neededApples <= 10<sup>15</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut x = 1;

        while 4 * x * x * x + 6 * x * x + 2 * x < needed_apples {
            x += 1;
        }

        8 * x
    }
}
```
