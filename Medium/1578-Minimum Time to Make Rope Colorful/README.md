# 1578. Minimum Time to Make Rope Colorful
Alice has `n` balloons arranged on a rope. You are given a **0-indexed** string `colors` where `colors[i]` is the color of the `ith` balloon.

Alice wants the rope to be **colorful**. She does not want **two consecutive balloons** to be of the same color, so she asks Bob for help. Bob can remove some balloons from the rope to make it **colorful**. You are given a **0-indexed** integer array `neededTime` where `neededTime[i]` is the time (in seconds) that Bob needs to remove the `ith` balloon from the rope.

Return *the **minimum time** Bob needs to make the rope **colorful***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/13/ballon1.jpg)
<pre>
<strong>Input:</strong> colors = "abaac", neededTime = [1,2,3,4,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> In the above image, 'a' is blue, 'b' is red, and 'c' is green.
Bob can remove the blue balloon at index 2. This takes 3 seconds.
There are no longer two consecutive balloons of the same color. Total time = 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/13/balloon2.jpg)
<pre>
<strong>Input:</strong> colors = "abc", neededTime = [1,2,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The rope is already colorful. Bob does not need to remove any balloons from the rope.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/12/13/balloon3.jpg)
<pre>
<strong>Input:</strong> colors = "aabaa", neededTime = [1,2,3,4,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Bob will remove the ballons at indices 0 and 4. Each ballon takes 1 second to remove.
There are no longer two consecutive balloons of the same color. Total time = 1 + 1 = 2.
</pre>

#### Constraints:
* `n == colors.length == neededTime.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= neededTime[i] <= 10<sup>4</sup></code>
* `colors` contains only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut ret = 0;

        for i in 1..s.len() {
            if s[i] == s[prev] {
                if cost[i] < cost[prev] {
                    ret += cost[i];
                    continue;
                }
                ret += cost[prev];
            }
            prev = i;
        }

        ret
    }
}
```
