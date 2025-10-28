# 335. Self Crossing
You are given an array of integers `distance`.

You start at the point `(0, 0)` on an **X-Y plane**, and you move `distance[0]` meters to the north, then `distance[1]` meters to the west, `distance[2]` meters to the south, `distance[3]` meters to the east, and so on. In other words, after each move, your direction changes counter-clockwise.

Return `true` *if your path crosses itself or* `false` *if it does not*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/12/21/11.jpg)
<pre>
<strong>Input:</strong> distance = [2,1,1,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> The path crosses itself at the point (0, 1).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/12/21/22.jpg)
<pre>
<strong>Input:</strong> distance = [1,2,3,4]
<strong>Output:</strong> false
<strong>Explanation:</strong> The path does not cross itself at any point.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/12/21/33.jpg)
<pre>
<strong>Input:</strong> distance = [1,1,1,2,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> The path crosses itself at the point (0, 0).
</pre>

#### Constraints:
* <code>1 <= distance.length <= 10<sup>5</sup></code>
* <code>1 <= distance[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_self_crossing(mut distance: Vec<i32>) -> bool {
        let mut i = 2;

        while i < distance.len() {
            if distance[i] <= distance[i - 2] {
                let mut tmp = distance[i - 2];
                if i > 3 {
                    tmp -= distance[i - 4];
                }
                if i > 2 && distance[i] >= tmp {
                    distance[i - 1] -= distance[i - 3];
                }
                break;
            }

            i += 1;
        }

        while i < distance.len() - 1 {
            i += 1;

            if distance[i] >= distance[i - 2] {
                return true;
            }
        }

        false
    }
}
```
