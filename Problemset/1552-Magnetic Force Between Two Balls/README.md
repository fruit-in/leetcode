# 1552. Magnetic Force Between Two Balls
In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they are put in his new invented basket. Rick has `n` empty baskets, the <code>i<sup>th</sup></code> basket is at `position[i]`, Morty has `m` balls and needs to distribute the balls into the baskets such that the **minimum magnetic force** between any two balls is **maximum**.

Rick stated that magnetic force between two different balls at positions `x` and `y` is `|x - y|`.

Given the integer array `position` and the integer `m`. Return *the required force*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/08/11/q3v1.jpg)
<pre>
<strong>Input:</strong> position = [1,2,3,4,7], m = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> position = [5,4,3,2,1,1000000000], m = 2
<strong>Output:</strong> 999999999
<strong>Explanation:</strong> We can use baskets 1 and 1000000000.
</pre>

#### Constraints:
* `n == position.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= position[i] <= 10<sup>9</sup></code>
* All integers in `position` are **distinct**.
* `2 <= m <= position.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        let mut lo = 1;
        let mut hi = *position.iter().max().unwrap();

        position.sort_unstable();

        while lo < hi {
            let force = (lo + hi + 1) / 2;
            let mut last = -force;
            let mut count = 0;

            for i in 0..position.len() {
                if position[i] - last >= force {
                    last = position[i];
                    count += 1;
                }
            }

            if count >= m {
                lo = force;
            } else {
                hi = force - 1;
            }
        }

        hi
    }
}
```
