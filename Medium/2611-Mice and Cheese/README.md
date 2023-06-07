# 2611. Mice and Cheese
There are two mice and `n` different types of cheese, each type of cheese should be eaten by exactly one mouse.

A point of the cheese with index `i` (**0-indexed**) is:

* `reward1[i]` if the first mouse eats it.
* `reward2[i]` if the second mouse eats it.

You are given a positive integer array `reward1`, a positive integer array `reward2`, and a non-negative integer `k`.

Return ***the maximum** points the mice can achieve if the first mouse eats exactly* `k` *types of cheese*.

#### Example 1:
<pre>
<strong>Input:</strong> reward1 = [1,1,3,4], reward2 = [4,4,1,1], k = 2
<strong>Output:</strong> 15
<strong>Explanation:</strong> In this example, the first mouse eats the 2nd (0-indexed) and the 3rd types of cheese, and the second mouse eats the 0th and the 1st types of cheese.
The total points are 4 + 4 + 3 + 4 = 15.
It can be proven that 15 is the maximum total points that the mice can achieve.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> reward1 = [1,1], reward2 = [1,1], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> In this example, the first mouse eats the 0th (0-indexed) and 1st types of cheese, and the second mouse does not eat any cheese.
The total points are 1 + 1 = 2.
It can be proven that 2 is the maximum total points that the mice can achieve.
</pre>

#### Constraints:
* <code>1 <= n == reward1.length == reward2.length <= 10<sup>5</sup></code>
* `1 <= reward1[i], reward2[i] <= 1000`
* `0 <= k <= n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut indices = (0..reward1.len()).collect::<Vec<_>>();
        let mut ret = 0;

        indices.sort_unstable_by_key(|&i| reward2[i] - reward1[i]);

        for i in 0..reward1.len() {
            if i < k as usize {
                ret += reward1[indices[i]];
            } else {
                ret += reward2[indices[i]];
            }
        }

        ret
    }
}
```
