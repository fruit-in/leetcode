# 1014. Best Sightseeing Pair
Given an array `A` of positive integers, `A[i]` represents the value of the `i`-th sightseeing spot, and two sightseeing spots `i` and `j` have distance `j - i` between them.

The *score* of a pair (`i < j`) of sightseeing spots is (`A[i] + A[j] + i - j`) : the sum of the values of the sightseeing spots, **minus** the distance between them.

Return the maximum score of a pair of sightseeing spots.

#### Example 1:
<pre>
<b>Input:</b> [8,1,5,2,6]
<b>Output:</b> 11
<b>Explanation:</b> i = 0, j = 2, A[i] + A[j] + i - j = 8 + 5 + 0 - 2 = 11
</pre>

#### Note:
1. `2 <= A.length <= 50000`
2. `1 <= A[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let mut score = a[0];
        let mut ret = 0;

        for j in 1..a.len() {
            ret = ret.max(score + a[j] - j as i32);
            score = score.max(a[j] + j as i32);
        }

        ret
    }
}
```
