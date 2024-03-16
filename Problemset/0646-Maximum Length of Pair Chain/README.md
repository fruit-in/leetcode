# 646. Maximum Length of Pair Chain
You are given an array of `n` pairs `pairs` where <code>pairs[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> and <code>left<sub>i</sub> < right<sub>i</sub></code>.

A pair `p2 = [c, d]` **follows** a pair `p1 = [a, b]` if `b < c`. A **chain** of pairs can be formed in this fashion.

Return *the length longest chain which can be formed*.

You do not need to use up all the given intervals. You can select pairs in any order.

#### Example 1:
<pre>
<strong>Input:</strong> pairs = [[1,2],[2,3],[3,4]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The longest chain is [1,2] -> [3,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> pairs = [[1,2],[7,8],[4,5]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest chain is [1,2] -> [4,5] -> [7,8].
</pre>

#### Constraints:
* `n == pairs.length`
* `1 <= n <= 1000`
* <code>-1000 <= left<sub>i</sub> < right<sub>i</sub> <= 1000</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| (p[1], -p[0]));
        let mut right = pairs[0][1];
        let mut ret = 1;

        for i in 1..pairs.len() {
            if pairs[i][0] > right {
                right = pairs[i][1];
                ret += 1;
            }
        }

        ret
    }
}
```
